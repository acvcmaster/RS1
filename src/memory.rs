use crate::bios::Bios;

#[derive(Debug, Clone)]
pub struct Memory {
    pub ram: Vec<u8>,                // 2048K (0x00000000, 0x80000000, 0xa0000000)
    pub expansion_region_1: Vec<u8>, // 8192K (0x1f000000, 0x9f000000, 0xbf000000)
    pub scratchpad: Vec<u8>,         // 1K (0x1f800000, 0x9f800000, 0xbf800000)
    pub hardware_registers: Vec<u8>, // 8K (0x1f801000, 0x9f801000, 0xbf801000)
    pub bios: Bios,               // 512K (0x1fc00000, 0x9fc00000, 0xbfc00000)
    pub io_ports: Vec<u8>,           // 512B (0xfffe0000)
}

pub enum MemoryRegion {
    Ram,
    ExpansionRegion,
    Scratchpad,
    HardwareRegisters,
    Bios,
    IOPorts,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ram: Vec::new(),
            expansion_region_1: Vec::new(),
            scratchpad: Vec::new(),
            hardware_registers: Vec::new(),
            bios: Bios::default(),
            io_ports: Vec::new(),
        }
    }

    pub fn load32(&self, address: u32) -> u32 {
        let region = Self::acquireRegion(address);

        match region.0 {
            MemoryRegion::Ram => 0,
            MemoryRegion::ExpansionRegion => 0,
            MemoryRegion::Scratchpad => 0,
            MemoryRegion::HardwareRegisters => 0,
            MemoryRegion::Bios => self.bios.load32(address - region.1),
            MemoryRegion::IOPorts => 0
        }
    }

    /// Acquires the memory region and the starting address
    pub fn acquireRegion(address: u32) -> (MemoryRegion, u32) {
        (MemoryRegion::Bios, 0xbfc00000)
    }

    pub fn loadBios(&mut self, bios: Bios) {
        self.bios = bios;
    }
}
