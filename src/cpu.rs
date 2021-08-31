use crate::{
    bios::Bios,
    decoded_instruction::{DecodedInstruction, EOpType, FOpType, IOpType, JOpType, ROpType},
    generic_error::GenericError,
    logger::handle_critical_result,
    memory::Memory,
};

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
        let instruction = self.fetch32(pc);

        self.pc = pc.wrapping_add(4);

        let result = self.decode_and_execute(instruction);
        handle_critical_result(result, Some("Instruction processing error:"));
    }

    pub fn decode_and_execute(&self, instruction: u32) -> Result<(), GenericError> {
        let decoded_instruction = DecodedInstruction::from(instruction);

        match decoded_instruction {
            DecodedInstruction::R {
                op,
                rs,
                rt,
                rd,
                shamt,
            } => match op {
                ROpType::Add => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Addu => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::And => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Brk => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Div => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Jalr => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Jr => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Mfhi => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Mflo => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Mult => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Nop => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Nor => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Or => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Sll => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Slt => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Sltu => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Srl => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Sub => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Subu => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Syscall => self.instruction_error(format!("{:?}", op), instruction, None),
                ROpType::Xor => self.instruction_error(format!("{:?}", op), instruction, None),
            },
            DecodedInstruction::I { op, rs, rt, imm } => match op {
                IOpType::Addi => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Addiu => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Andi => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Beq => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Blez => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Bne => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Lb => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Lbu => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Ldc1 => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Lh => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Lhu => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Lui => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Lw => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Lwc1 => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Ori => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Sb => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Sh => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Slti => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Sltiu => self.instruction_error(format!("{:?}", op), instruction, None),
                IOpType::Sw => self.instruction_error(format!("{:?}", op), instruction, None),
            },
            DecodedInstruction::J { op, addr } => match op {
                JOpType::J => self.instruction_error(format!("{:?}", op), instruction, None),
                JOpType::Jal => self.instruction_error(format!("{:?}", op), instruction, None),
            },
            DecodedInstruction::F { op, rt, rs, rd } => match op {
                FOpType::Addd => self.instruction_error(format!("{:?}", op), instruction, None),
                FOpType::Adds => self.instruction_error(format!("{:?}", op), instruction, None),
                FOpType::Cvtdw => self.instruction_error(format!("{:?}", op), instruction, None),
                FOpType::Cvtsd => self.instruction_error(format!("{:?}", op), instruction, None),
                FOpType::Divd => self.instruction_error(format!("{:?}", op), instruction, None),
                FOpType::Divs => self.instruction_error(format!("{:?}", op), instruction, None),
                FOpType::Mfc1 => self.instruction_error(format!("{:?}", op), instruction, None),
                FOpType::Movd => self.instruction_error(format!("{:?}", op), instruction, None),
                FOpType::Movs => self.instruction_error(format!("{:?}", op), instruction, None),
                FOpType::Mtc1 => self.instruction_error(format!("{:?}", op), instruction, None),
                FOpType::Muld => self.instruction_error(format!("{:?}", op), instruction, None),
                FOpType::Muls => self.instruction_error(format!("{:?}", op), instruction, None),
            },
            DecodedInstruction::E { op, instruction } => match op {
                EOpType::Unknown => {
                    self.instruction_error(format!("{:?}", op), instruction, Some(true))
                }
            },
        }
    }

    pub fn instruction_error(
        &self,
        op: String,
        instruction: u32,
        unknown: Option<bool>,
    ) -> Result<(), GenericError> {
        Err(GenericError {
            message: if unknown == None {
                format!(
                    "UNIMPLEMENTED_INSTRUCTION_{} (0x{:08x} at 0x{:08x})",
                    op,
                    instruction,
                    self.pc.wrapping_sub(4)
                )
            } else {
                format!(
                    "UNKNOWN_INSTRUCTION (found 0x{:08x} at 0x{:08x})",
                    instruction,
                    self.pc.wrapping_sub(4)
                )
            },
        })
    }

    pub fn fetch32(&self, address: u32) -> u32 {
        let result = self.memory.load32(address);
        handle_critical_result(result, Some("Instruction fetch error:"))
    }

    pub fn load_bios(&mut self, bios: Bios) {
        self.memory.load_bios(bios);
    }
}
