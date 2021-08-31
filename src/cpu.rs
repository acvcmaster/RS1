use crate::{bios::Bios, generic_error::GenericError, logger::{handle_critical_result, handle_result}, memory::Memory};

#[derive(Debug, Clone)]
pub struct Cpu {
    pub pc: u32,
    pub memory: Memory,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0xbfc00000,
            memory: Memory::new(),
        }
    }

    pub fn run_next_instruction(&mut self) {
        let pc = self.pc;

        let result = self.memory.load32(pc);
        let instruction =
            handle_critical_result(result, Some("Instruction fetch error:".to_string()));

        self.pc = pc.wrapping_add(4);
        
        let result = self.decode_and_execute(instruction);
        handle_critical_result(result, Some("Instruction execution error:".to_string()));
    }

    pub fn decode_and_execute(&mut self, instruction: u32) -> Result<(), GenericError> {
        // mock
        Err(GenericError { message: format!("UNKNOWN_INSTRUCTION (found 0x{:08x} at 0x{:08x})", instruction, self.pc) })
    }

    pub fn load_bios(&mut self, bios: Bios) {
        self.memory.load_bios(bios);
    }
}
