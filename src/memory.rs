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
            ram: vec![0; 2048 * 1024],
            expansion_region_1: vec![0; 8192 * 1024],
            scratchpad: vec![0; 1024],
            hardware_registers: vec![0; 8 * 1024],
            bios: Bios::default(),
            io_ports: vec![0; 512],
        }
    }

    pub fn load32(&self, address: u32) -> Result<u32, GenericError> {
        if address % 4 != 0 {
            return Err(GenericError {
                message: format!("LOAD32_UNALIGNED_WORD_BOUNDARY (from 0x{:x})", address),
            });
        }

        for i in 0..16 {
            let region = REGIONS[i];

            match region.contains(address) {
                Some(offset) => {
                    return Ok(match region.2 {
                        MemoryRegionType::RAM => Memory::load_generic(&self.ram, offset),
                        MemoryRegionType::ExpansionRegion => {
                            Memory::load_generic(&self.expansion_region_1, offset)
                        }
                        MemoryRegionType::Scratchpad => {
                            Memory::load_generic(&self.scratchpad, offset)
                        }
                        MemoryRegionType::HardwareRegisters => {
                            Memory::load_generic(&self.hardware_registers, offset)
                        }
                        MemoryRegionType::BIOS => self.bios.load32(offset),
                        MemoryRegionType::IOPorts => Memory::load_generic(&self.io_ports, offset),
                        MemoryRegionType::MemlControl => handle_memlcontrol_read(offset)
                    })
                }
                None => continue,
            }
        }

        Err(GenericError {
            message: format!("LOAD32_PERIPHERAL_NOT_FOUND"),
        })
    }

    pub fn store32(&mut self, address: u32, word: u32) -> Result<(), GenericError> {
        if address % 4 != 0 {
            return Err(GenericError {
                message: format!("STORE32_UNALIGNED_WORD_BOUNDARY (into 0x{:x})", address),
            });
        }

        for i in 0..REGIONS.len() {
            let region = REGIONS[i];

            match region.contains(address) {
                Some(offset) => {
                    return Ok(match region.2 {
                        MemoryRegionType::RAM => Memory::store_generic(&mut self.ram, offset, word),
                        MemoryRegionType::ExpansionRegion => {
                            Memory::store_generic(&mut self.expansion_region_1, offset, word)
                        }
                        MemoryRegionType::Scratchpad => {
                            Memory::store_generic(&mut self.scratchpad, offset, word)
                        }
                        MemoryRegionType::HardwareRegisters => {
                            Memory::store_generic(&mut self.hardware_registers, offset, word)
                        }
                        MemoryRegionType::BIOS => (), // BIOS is read-only
                        MemoryRegionType::IOPorts => {
                            Memory::store_generic(&mut self.io_ports, offset, word)
                        }
                        MemoryRegionType::MemlControl => if let Some(value) = handle_memlcontrol_store(offset, word) {
                            return value;
                        },
                    });
                }
                None => continue,
            }
        }

        Err(GenericError {
            message: format!("STORE32_PERIPHERAL_NOT_FOUND"),
        })
    }

    pub fn load_bios(&mut self, bios: Bios) {
        self.bios = bios;
    }

    pub fn load_generic(data: &Vec<u8>, offset: u32) -> u32 {
        let offset = offset as usize;
        let b0 = data[offset + 0] as u32;
        let b1 = data[offset + 1] as u32;
        let b2 = data[offset + 2] as u32;
        let b3 = data[offset + 3] as u32;
        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
    }

    pub fn store_generic(data: &mut Vec<u8>, offset: u32, word: u32) {
        let address = offset as usize;

        let b0 = (word & 0xff) as u8;
        let b1 = ((word & 0xff00) >> 8) as u8;
        let b2 = ((word & 0xff0000) >> 16) as u8;
        let b3 = ((word & 0xff000000) >> 24) as u8;

        data[address + 0] = b0;
        data[address + 1] = b1;
        data[address + 2] = b2;
        data[address + 3] = b3;
    }
}

fn handle_memlcontrol_store(offset: u32, word: u32) -> Option<Result<(), GenericError>> {
    match offset {
        0 => {
            if word != 0x1f000000 {
                return Some(Err(GenericError {
                    message: format!(
                        "STORE32_BAD_EXPANSION_1_BASE_ADDRESS (0x{:x})",
                        word
                    ),
                }));
            }
        }
        4 => {
            if word != 0x1f802000 {
                return Some(Err(GenericError {
                    message: format!(
                        "STORE32_BAD_EXPANSION_2_BASE_ADDRESS (0x{:x})",
                        word
                    ),
                }));
            }
        }
        _ => {
            return Some(Err(GenericError {
                message: format!(
                    "STORE32_UNHANDLED_MEMLCONTROL_WRITE (0x{:x})",
                    word
                ),
            }));
        }
    }
    None
}

fn handle_memlcontrol_read(offset: u32) -> u32 {
    match offset {
        0 => 0x1f000000,
        4 => 0x1f802000,
        _ => 0
    }
}