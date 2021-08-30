use crate::{bios::Bios, memory::Memory};

#[derive(Debug, Clone)]
pub struct Cpu {
    pub pc: u32,
    pub memory: Memory
}

impl Cpu {
    pub fn new() -> Self {
        Self { pc: 0xbfc00000, memory: Memory::new() }
    }

    pub fn run_next_instruction(&mut self) {
        let pc = self.pc;

        let instruction = self.memory.load32(pc);
        self.pc = pc.wrapping_add(4);
        self.decode_and_execute(instruction);
    }

    pub fn decode_and_execute(&mut self, instruction: u32) {}

    pub fn load_bios(&mut self, bios: Bios) {
        self.memory.loadBios(bios);
    }
}
