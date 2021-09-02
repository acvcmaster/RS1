use crate::{
    bios::Bios,
    decoded_instruction::{DecodedInstruction, EOpType, FOpType, IOpType, JOpType, ROpType},
    generic_error::GenericError,
    logger::{handle_critical_result, handle_result},
    memory::Memory,
};

#[derive(Debug, Clone)]
pub struct Cpu {
    pub pc: u32,
    pub memory: Memory,
    pub gpr: [u32; 32], // General Purpose Registers ($0 - $31)
    pub branch_delay_slot: u32,
    pub sr: u32,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0xbfc00000,
            memory: Memory::new(),
            gpr: [0; 32],
            branch_delay_slot: 0x00000000, // nop
            sr: 0x00000000,
        }
    }

    pub fn run_next_instruction(&mut self, print: bool) {
        let pc = self.pc;
        let instruction = self.load32(pc);

        self.pc = pc.wrapping_add(4);

        self.branch_delay_slot = self.load32(self.pc);

        let result = self.decode_and_execute(instruction, print);
        handle_critical_result(result, Some("Instruction processing error:"));
    }

    pub fn decode_and_execute(
        &mut self,
        instruction: u32,
        print: bool,
    ) -> Result<(), GenericError> {
        let decoded_instruction = DecodedInstruction::from(instruction);

        match decoded_instruction {
            DecodedInstruction::R {
                op,
                rs,
                rt,
                rd,
                shamt,
            } => match op {
                ROpType::Add => self.instruction_error(format!("{:?}", op), instruction, false),
                ROpType::Addu => self.instruction_error(format!("{:?}", op), instruction, false),
                ROpType::And => self.instruction_error(format!("{:?}", op), instruction, false),
                ROpType::Brk => self.instruction_error(format!("{:?}", op), instruction, false),
                ROpType::Div => self.instruction_error(format!("{:?}", op), instruction, false),
                ROpType::Jalr => self.instruction_error(format!("{:?}", op), instruction, false),
                ROpType::Jr => self.instruction_error(format!("{:?}", op), instruction, false),
                ROpType::Mfhi => self.instruction_error(format!("{:?}", op), instruction, false),
                ROpType::Mflo => self.instruction_error(format!("{:?}", op), instruction, false),
                ROpType::Mult => self.instruction_error(format!("{:?}", op), instruction, false),
                ROpType::Nop => self.nop(print),
                ROpType::Nor => self.instruction_error(format!("{:?}", op), instruction, false),
                ROpType::Or => self.or(rs, rt, rd, print),
                ROpType::Sll => self.sll(rt, rd, shamt, print),
                ROpType::Slt => self.instruction_error(format!("{:?}", op), instruction, false),
                ROpType::Sltu => self.instruction_error(format!("{:?}", op), instruction, false),
                ROpType::Srl => self.srl(rt, rd, shamt, print),
                ROpType::Sub => self.instruction_error(format!("{:?}", op), instruction, false),
                ROpType::Subu => self.instruction_error(format!("{:?}", op), instruction, false),
                ROpType::Syscall => self.instruction_error(format!("{:?}", op), instruction, false),
                ROpType::Xor => self.instruction_error(format!("{:?}", op), instruction, false),
            },
            DecodedInstruction::I { op, rs, rt, imm } => match op {
                IOpType::Addi => self.addi(rt, rs, imm, print),
                IOpType::Addiu => self.addiu(rt, rs, imm, print),
                IOpType::Andi => self.instruction_error(format!("{:?}", op), instruction, false),
                IOpType::Beq => self.instruction_error(format!("{:?}", op), instruction, false),
                IOpType::Blez => self.instruction_error(format!("{:?}", op), instruction, false),
                IOpType::Bne => self.bne(rs, rt, imm, print),
                IOpType::Lb => self.instruction_error(format!("{:?}", op), instruction, false),
                IOpType::Lbu => self.instruction_error(format!("{:?}", op), instruction, false),
                IOpType::Ldc1 => self.instruction_error(format!("{:?}", op), instruction, false),
                IOpType::Lh => self.instruction_error(format!("{:?}", op), instruction, false),
                IOpType::Lhu => self.instruction_error(format!("{:?}", op), instruction, false),
                IOpType::Lui => self.lui(rt, imm, print),
                IOpType::Lw => self.instruction_error(format!("{:?}", op), instruction, false),
                IOpType::Lwc1 => self.instruction_error(format!("{:?}", op), instruction, false),
                IOpType::Ori => self.ori(rt, rs, imm, print),
                IOpType::Sb => self.instruction_error(format!("{:?}", op), instruction, false),
                IOpType::Sh => self.instruction_error(format!("{:?}", op), instruction, false),
                IOpType::Slti => self.instruction_error(format!("{:?}", op), instruction, false),
                IOpType::Sltiu => self.instruction_error(format!("{:?}", op), instruction, false),
                IOpType::Sw => self.sw(rs, rt, imm, print),
            },
            DecodedInstruction::J { op, addr } => match op {
                JOpType::J => self.j(addr, print),
                JOpType::Jal => self.instruction_error(format!("{:?}", op), instruction, false),
            },
            DecodedInstruction::F { op, rt, rs, rd } => match op {
                FOpType::Addd => self.instruction_error(format!("{:?}", op), instruction, false),
                FOpType::Adds => self.instruction_error(format!("{:?}", op), instruction, false),
                FOpType::Cvtdw => self.instruction_error(format!("{:?}", op), instruction, false),
                FOpType::Cvtsd => self.instruction_error(format!("{:?}", op), instruction, false),
                FOpType::Divd => self.instruction_error(format!("{:?}", op), instruction, false),
                FOpType::Divs => self.instruction_error(format!("{:?}", op), instruction, false),
                FOpType::Mfc1 => self.instruction_error(format!("{:?}", op), instruction, false),
                FOpType::Movd => self.instruction_error(format!("{:?}", op), instruction, false),
                FOpType::Movs => self.instruction_error(format!("{:?}", op), instruction, false),
                FOpType::Mtc1 => self.instruction_error(format!("{:?}", op), instruction, false),
                FOpType::Muld => self.instruction_error(format!("{:?}", op), instruction, false),
                FOpType::Muls => self.instruction_error(format!("{:?}", op), instruction, false),
                FOpType::Mtc0 => self.mtc0(rt, rs, print),
            },
            DecodedInstruction::E { op, instruction } => match op {
                EOpType::Unknown => self.instruction_error(format!("{:?}", op), instruction, true),
            },
        }
    }

    pub fn instruction_error(
        &self,
        op: String,
        instruction: u32,
        unknown: bool,
    ) -> Result<(), GenericError> {
        Err(GenericError {
            message: if unknown {
                format!(
                    "UNKNOWN_INSTRUCTION (found 0x{:08x} at 0x{:08x})",
                    instruction,
                    self.pc.wrapping_sub(4)
                )
            } else {
                format!(
                    "UNIMPLEMENTED_INSTRUCTION_{} (0x{:08x} at 0x{:08x})",
                    op,
                    instruction,
                    self.pc.wrapping_sub(4)
                )
            },
        })
    }

    pub fn load32(&self, address: u32) -> u32 {
        let result = self.memory.load32(address);
        handle_critical_result(result, Some("Instruction fetch error:"))
    }

    pub fn store32(&mut self, address: u32, word: u32) {
        let message = Some("Memory write soft error:");
        let result = if self.sr & 0x10000 != 0 {
            Err(GenericError {
                message: format!("IGNORE_WRITE_ON_ISOLATED_CACHE (at 0x{:x})", self.pc - 4),
            })
        } else {
            Ok(())
        };

        if result.is_err() {
            handle_result(result, message);
        } else {
            let result = self.memory.store32(address, word);
            handle_critical_result(result, message)
        }
    }

    pub fn load_bios(&mut self, bios: Bios) {
        self.memory.load_bios(bios);
    }

    pub fn reg(&self, index: u32) -> u32 {
        self.gpr[index as usize]
    }

    pub fn set_reg(&mut self, index: u32, value: u32) {
        self.gpr[index as usize] = value;
        self.gpr[0] = 0; // $0 is always zero
    }
}
