//! Experimental MultiVM lifecycle and guest-memory model.
//!
//! Each VM owns a separate byte buffer. This models separation inside one host
//! process; it is not equivalent to process, hypervisor, or hardware isolation.

use aegis_cap::CapabilityTable;

/// Unique identifier assigned by the runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VmId(u64);

impl VmId {
    pub fn get(self) -> u64 { self.0 }
}

/// Runtime configuration for one guest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VmConfig {
    pub memory_bytes: usize,
}

/// Errors returned by the VM manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmError {
    ZeroMemory,
    MemoryLimitExceeded,
    UnknownVm,
    AddressOutOfBounds,
}

/// One VM's private prototype execution context and guest memory.
#[derive(Debug)]
pub struct MicroVm {
    id: VmId,
    memory: Vec<u8>,
    registers: [u64; 32],
    capabilities: CapabilityTable,
}

impl MicroVm {
    pub fn id(&self) -> VmId { self.id }
    pub fn memory_len(&self) -> usize { self.memory.len() }
    pub fn capabilities(&self) -> &CapabilityTable { &self.capabilities }
    pub fn capabilities_mut(&mut self) -> &mut CapabilityTable { &mut self.capabilities }

    pub fn read_guest(&self, address: usize, output: &mut [u8]) -> Result<(), VmError> {
        let end = address.checked_add(output.len()).ok_or(VmError::AddressOutOfBounds)?;
        let source = self.memory.get(address..end).ok_or(VmError::AddressOutOfBounds)?;
        output.copy_from_slice(source);
        Ok(())
    }

    pub fn write_guest(&mut self, address: usize, input: &[u8]) -> Result<(), VmError> {
        let end = address.checked_add(input.len()).ok_or(VmError::AddressOutOfBounds)?;
        let destination = self.memory.get_mut(address..end).ok_or(VmError::AddressOutOfBounds)?;
        destination.copy_from_slice(input);
        Ok(())
    }

    pub fn register(&self, index: usize) -> Option<u64> { self.registers.get(index).copied() }
}

/// Creates and tracks independent VM instances.
#[derive(Debug)]
pub struct VmManager {
    vms: Vec<MicroVm>,
    next_id: u64,
    max_vm_memory: usize,
}

impl VmManager {
    pub fn new(max_vm_memory: usize) -> Self {
        Self { vms: Vec::new(), next_id: 1, max_vm_memory }
    }

    pub fn create(&mut self, config: VmConfig) -> Result<VmId, VmError> {
        if config.memory_bytes == 0 { return Err(VmError::ZeroMemory); }
        if config.memory_bytes > self.max_vm_memory { return Err(VmError::MemoryLimitExceeded); }
        let id = VmId(self.next_id);
        self.next_id = self.next_id.checked_add(1).expect("VM identifier space exhausted");
        self.vms.push(MicroVm {
            id,
            memory: vec![0; config.memory_bytes],
            registers: [0; 32],
            capabilities: CapabilityTable::new(),
        });
        Ok(id)
    }

    pub fn get(&self, id: VmId) -> Result<&MicroVm, VmError> {
        self.vms.iter().find(|vm| vm.id == id).ok_or(VmError::UnknownVm)
    }

    pub fn get_mut(&mut self, id: VmId) -> Result<&mut MicroVm, VmError> {
        self.vms.iter_mut().find(|vm| vm.id == id).ok_or(VmError::UnknownVm)
    }

    pub fn destroy(&mut self, id: VmId) -> Result<(), VmError> {
        let index = self.vms.iter().position(|vm| vm.id == id).ok_or(VmError::UnknownVm)?;
        self.vms.swap_remove(index);
        Ok(())
    }

    pub fn count(&self) -> usize { self.vms.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guests_have_independent_memory() {
        let mut manager = VmManager::new(4096);
        let first = manager.create(VmConfig { memory_bytes: 64 }).unwrap();
        let second = manager.create(VmConfig { memory_bytes: 64 }).unwrap();
        manager.get_mut(first).unwrap().write_guest(0, b"AEGIS").unwrap();
        let mut bytes = [0; 5];
        manager.get(second).unwrap().read_guest(0, &mut bytes).unwrap();
        assert_eq!(&bytes, &[0; 5]);
    }

    #[test]
    fn enforces_memory_limit_and_bounds() {
        let mut manager = VmManager::new(32);
        assert_eq!(manager.create(VmConfig { memory_bytes: 33 }), Err(VmError::MemoryLimitExceeded));
        let id = manager.create(VmConfig { memory_bytes: 32 }).unwrap();
        assert_eq!(manager.get(id).unwrap().read_guest(31, &mut [0; 2]), Err(VmError::AddressOutOfBounds));
    }
}
