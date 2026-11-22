//! Capability-table prototype.
//!
//! Handles are opaque to callers. This is an in-process software model, not a
//! hardware-enforced capability system or a complete security boundary.

/// Rights represented as a compact bitset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rights(u64);

impl Rights {
    pub const READ: Self = Self(1 << 0);
    pub const WRITE: Self = Self(1 << 1);
    pub const EXECUTE: Self = Self(1 << 2);
    pub const SEND: Self = Self(1 << 3);
    pub const RECEIVE: Self = Self(1 << 4);
    pub const ADMIN: Self = Self(1 << 5);

    pub const fn empty() -> Self { Self(0) }
    pub const fn bits(self) -> u64 { self.0 }
    pub const fn contains(self, other: Self) -> bool { (self.0 & other.0) == other.0 }
    pub const fn is_subset_of(self, other: Self) -> bool { (self.0 & !other.0) == 0 }
}

impl core::ops::BitOr for Rights {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output { Self(self.0 | rhs.0) }
}

/// Opaque handle. Fields are private so callers cannot construct arbitrary handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Capability {
    slot: usize,
    generation: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Entry {
    object: u128,
    rights: Rights,
    start: u64,
    end: u64,
    generation: u64,
    live: bool,
}

/// Errors from capability operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityError {
    InvalidHandle,
    InvalidBounds,
    RightsEscalation,
}

/// In-memory capability registry for the prototype runtime.
#[derive(Debug, Default)]
pub struct CapabilityTable {
    entries: Vec<Entry>,
}

impl CapabilityTable {
    pub fn new() -> Self { Self::default() }

    pub fn issue(&mut self, object: u128, rights: Rights, start: u64, end: u64) -> Result<Capability, CapabilityError> {
        if start > end { return Err(CapabilityError::InvalidBounds); }
        let slot = self.entries.len();
        self.entries.push(Entry { object, rights, start, end, generation: 1, live: true });
        Ok(Capability { slot, generation: 1 })
    }

    pub fn derive(&mut self, parent: Capability, rights: Rights, start: u64, end: u64) -> Result<Capability, CapabilityError> {
        let source = self.resolve(parent)?;
        if !rights.is_subset_of(source.rights) || start < source.start || end > source.end || start > end {
            return Err(CapabilityError::RightsEscalation);
        }
        self.issue(source.object, rights, start, end)
    }

    pub fn check(&self, cap: Capability, required: Rights, start: u64, end: u64) -> Result<u128, CapabilityError> {
        let entry = self.resolve(cap)?;
        if !entry.rights.contains(required) || start < entry.start || end > entry.end || start > end {
            return Err(CapabilityError::RightsEscalation);
        }
        Ok(entry.object)
    }

    pub fn revoke(&mut self, cap: Capability) -> Result<(), CapabilityError> {
        let entry = self.entries.get_mut(cap.slot).ok_or(CapabilityError::InvalidHandle)?;
        if !entry.live || entry.generation != cap.generation {
            return Err(CapabilityError::InvalidHandle);
        }
        entry.live = false;
        entry.generation = entry.generation.wrapping_add(1);
        Ok(())
    }

    fn resolve(&self, cap: Capability) -> Result<&Entry, CapabilityError> {
        let entry = self.entries.get(cap.slot).ok_or(CapabilityError::InvalidHandle)?;
        if !entry.live || entry.generation != cap.generation {
            return Err(CapabilityError::InvalidHandle);
        }
        Ok(entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_capability_cannot_gain_rights_or_bounds() {
        let mut table = CapabilityTable::new();
        let root = table.issue(7, Rights::READ, 10, 100).unwrap();
        assert!(table.derive(root, Rights::READ | Rights::WRITE, 10, 20).is_err());
        assert!(table.derive(root, Rights::READ, 0, 20).is_err());
        let child = table.derive(root, Rights::READ, 20, 30).unwrap();
        assert_eq!(table.check(child, Rights::READ, 20, 30), Ok(7));
    }

    #[test]
    fn revoked_handle_is_invalid() {
        let mut table = CapabilityTable::new();
        let cap = table.issue(9, Rights::READ, 0, 8).unwrap();
        table.revoke(cap).unwrap();
        assert_eq!(table.check(cap, Rights::READ, 0, 1), Err(CapabilityError::InvalidHandle));
    }
}
