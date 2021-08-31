#[derive(Clone, Copy)]
pub struct MemoryRegion(pub u32, pub u32, pub MemoryRegionType);

#[derive(Clone, Copy)]
pub enum MemoryRegionType {
    RAM,
    ExpansionRegion,
    Scratchpad,
    HardwareRegisters,
    BIOS,
    IOPorts,
}

impl MemoryRegion {
    pub fn contains(self, address: u32) -> Option<u32> {
        let Self(start, length, _) = self;

        if address >= start && address < start + length {
            Some(address - start)
        } else {
            None
        }
    }
}

pub const RAM_1: MemoryRegion = MemoryRegion(0x00000000, 0x200000, MemoryRegionType::RAM);
pub const RAM_2: MemoryRegion = MemoryRegion(0x80000000, 0x200000, MemoryRegionType::RAM);
pub const RAM_3: MemoryRegion = MemoryRegion(0xa0000000, 0x200000, MemoryRegionType::RAM);

pub const EXPANSION_REGION_1_1: MemoryRegion =
    MemoryRegion(0x1f000000, 0x800000, MemoryRegionType::ExpansionRegion);
pub const EXPANSION_REGION_1_2: MemoryRegion =
    MemoryRegion(0x9f000000, 0x800000, MemoryRegionType::ExpansionRegion);
pub const EXPANSION_REGION_1_3: MemoryRegion =
    MemoryRegion(0xbf000000, 0x800000, MemoryRegionType::ExpansionRegion);

pub const SCRATCHPAD_1: MemoryRegion =
    MemoryRegion(0x1f800000, 0x400, MemoryRegionType::Scratchpad);
pub const SCRATCHPAD_2: MemoryRegion =
    MemoryRegion(0x9f800000, 0x400, MemoryRegionType::Scratchpad);
pub const SCRATCHPAD_3: MemoryRegion =
    MemoryRegion(0xbf800000, 0x400, MemoryRegionType::Scratchpad);

pub const HARDWARE_REGISTERS_1: MemoryRegion =
    MemoryRegion(0x1f801000, 0x2000, MemoryRegionType::HardwareRegisters);
pub const HARDWARE_REGISTERS_2: MemoryRegion =
    MemoryRegion(0x9f801000, 0x2000, MemoryRegionType::HardwareRegisters);
pub const HARDWARE_REGISTERS_3: MemoryRegion =
    MemoryRegion(0xbf801000, 0x2000, MemoryRegionType::HardwareRegisters);

pub const BIOS_1: MemoryRegion = MemoryRegion(0x1fc00000, 0x80000, MemoryRegionType::BIOS);
pub const BIOS_2: MemoryRegion = MemoryRegion(0x9fc00000, 0x80000, MemoryRegionType::BIOS);
pub const BIOS_3: MemoryRegion = MemoryRegion(0xbfc00000, 0x80000, MemoryRegionType::BIOS);

pub const IO_PORTS: MemoryRegion = MemoryRegion(0xfffe0000, 0x200, MemoryRegionType::IOPorts);

pub const REGIONS: [MemoryRegion; 16] = [
    RAM_1,
    RAM_2,
    RAM_3,
    EXPANSION_REGION_1_1,
    EXPANSION_REGION_1_2,
    EXPANSION_REGION_1_3,
    SCRATCHPAD_1,
    SCRATCHPAD_2,
    SCRATCHPAD_3,
    HARDWARE_REGISTERS_1,
    HARDWARE_REGISTERS_2,
    HARDWARE_REGISTERS_3,
    BIOS_1,
    BIOS_2,
    BIOS_3,
    IO_PORTS,
];
