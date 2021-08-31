use crate::{
    bios::Bios,
    generic_error::GenericError,
    memory_region::{MemoryRegionType, REGIONS},
};

#[derive(Debug, Clone)]
pub struct Memory {
    pub ram: Vec<u8>,                // 2048K (0x00000000, 0x80000000, 0xa0000000)
    pub expansion_region_1: Vec<u8>, // 8192K (0x1f000000, 0x9f000000, 0xbf000000)
    pub scratchpad: Vec<u8>,         // 1K (0x1f800000, 0x9f800000, 0xbf800000)
    pub hardware_registers: Vec<u8>, // 8K (0x1f801000, 0x9f801000, 0xbf801000)
    pub bios: Bios,                  // 512K (0x1fc00000, 0x9fc00000, 0xbfc00000)
    pub io_ports: Vec<u8>,           // 512B (0xfffe0000)
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

    pub fn load32(&self, address: u32) -> Result<u32, GenericError> {
        for i in 0..16 {
            let region = REGIONS[i];

            match region.contains(address) {
                Some(offset) => {
                    return Ok(match region.2 {
                        MemoryRegionType::RAM => 0,
                        MemoryRegionType::ExpansionRegion => 0,
                        MemoryRegionType::Scratchpad => 0,
                        MemoryRegionType::HardwareRegisters => 0,
                        MemoryRegionType::BIOS => self.bios.load32(offset),
                        MemoryRegionType::IOPorts => 0,
                    })
                }
                None => continue,
            }
        }

        Err(GenericError {
            message: "LOAD32_PERIPHERAL_NOT_FOUND".to_string(),
        })
    }

    pub fn store32(&self, address: u32, word: u32) -> Result<(), GenericError> {
        for i in 0..16 {
            let region = REGIONS[i];

            match region.contains(address) {
                Some(offset) => {
                    return Ok(match region.2 {
                        MemoryRegionType::RAM => (),
                        MemoryRegionType::ExpansionRegion => (),
                        MemoryRegionType::Scratchpad => (),
                        MemoryRegionType::HardwareRegisters => (),
                        MemoryRegionType::BIOS => (),
                        MemoryRegionType::IOPorts => (),
                    })
                }
                None => continue,
            }
        }

        Err(GenericError {
            message: "STORE32_PERIPHERAL_NOT_FOUND".to_string(),
        })
    }

    pub fn load_bios(&mut self, bios: Bios) {
        self.bios = bios;
    }
}
