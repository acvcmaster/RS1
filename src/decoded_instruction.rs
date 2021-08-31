#[derive(Debug, Clone, Copy)]
pub enum ROpType {
    Add,
    Addu,
    And,
    Brk,
    Div,
    Jalr,
    Jr,
    Mfhi,
    Mflo,
    Mult,
    Nop,
    Nor,
    Or,
    Sll,
    Slt,
    Sltu,
    Srl,
    Sub,
    Subu,
    Syscall,
    Xor,
}

#[derive(Debug, Clone, Copy)]
pub enum IOpType {
    Addi,
    Addiu,
    Andi,
    Beq,
    Blez,
    Bne,
    Lb,
    Lbu,
    Ldc1,
    Lh,
    Lhu,
    Lui,
    Lw,
    Lwc1,
    Ori,
    Sb,
    Sh,
    Slti,
    Sltiu,
    Sw,
}

#[derive(Debug, Clone, Copy)]
pub enum JOpType {
    J,
    Jal,
}

#[derive(Debug, Clone, Copy)]
pub enum FOpType {
    Addd,
    Adds,
    Cvtdw,
    Cvtsd,
    Divd,
    Divs,
    Mfc1,
    Movd,
    Movs,
    Mtc1,
    Muld,
    Muls,
}

#[derive(Debug, Clone, Copy)]
pub enum EOpType {
    Unknown,
}

pub enum DecodedInstruction {
    R {
        op: ROpType,
        rs: u32,
        rt: u32,
        rd: u32,
        shamt: u32,
    },
    I {
        op: IOpType,
        rs: u32,
        rt: u32,
        imm: u32,
    },
    J {
        op: JOpType,
        addr: i32,
    },
    F {
        op: FOpType,
        rt: u32,
        rs: u32,
        rd: u32,
    },
    E {
        op: EOpType,
        instruction: u32,
    },
}

impl DecodedInstruction {
    /// Constructs a new `R`-type instruction from a word.
    fn new_r(op: ROpType, word: u32) -> Self {
        Self::R {
            op,
            rs: ((word & 0x3e00000) >> 21) as u32,
            rt: ((word & 0x1f0000) >> 16) as u32,
            rd: ((word & 0xf800) >> 11) as u32,
            shamt: ((word & 0x7c0) >> 6) as u32,
        }
    }

    /// Constructs a new `I`-type instruction from a word.
    fn new_i(op: IOpType, word: u32) -> Self {
        Self::I {
            op,
            rs: ((word & 0x3e00000) >> 21) as u32,
            rt: ((word & 0x1f0000) >> 16) as u32,
            imm: (word & 0xffff) as u32,
        }
    }

    /// Constructs a new `J`-type instruction from a word.
    fn new_j(op: JOpType, word: i32) -> Self {
        Self::J {
            op,
            addr: word & 0x3ffffff,
        }
    }

    /// Constructs a new `F`-type instruction from a word.
    fn new_f(op: FOpType, word: u32) -> Self {
        Self::F {
            op,
            rt: ((word & 0x1f0000) >> 16) as u32,
            rs: ((word & 0xf800) >> 11) as u32,
            rd: ((word & 0x7c0) >> 6) as u32,
        }
    }

    /// Constructs a new `N`-type instruction from a word.
    fn new_e(op: EOpType, word: u32) -> Self {
        Self::E {
            op,
            instruction: word,
        }
    }
}

impl From<u32> for DecodedInstruction {
    fn from(word: u32) -> Self {
        use {EOpType::*, FOpType::*, IOpType::*, JOpType::*, ROpType::*};
        if word == 0 {
            return DecodedInstruction::new_r(Nop, word);
        }
        let opcode = (word as u32) >> 26;
        let funct = word & 0x3f;
        let fmt = ((word & 0x3e00000) >> 21) as usize;
        match (opcode, funct, fmt) {
            (0, 0x10, ..) => DecodedInstruction::new_r(Mfhi, word),
            (0, 0x12, ..) => DecodedInstruction::new_r(Mflo, word),
            (0, 0x18, ..) => DecodedInstruction::new_r(Mult, word),
            (0, 0x1A, ..) => DecodedInstruction::new_r(Div, word),
            (0, 0x20, ..) => DecodedInstruction::new_r(Add, word),
            (0, 0x21, ..) => DecodedInstruction::new_r(Addu, word),
            (0, 0x22, ..) => DecodedInstruction::new_r(Sub, word),
            (0, 0x23, ..) => DecodedInstruction::new_r(Subu, word),
            (0, 0x24, ..) => DecodedInstruction::new_r(And, word),
            (0, 0x25, ..) => DecodedInstruction::new_r(Or, word),
            (0, 0x26, ..) => DecodedInstruction::new_r(Xor, word),
            (0, 0x27, ..) => DecodedInstruction::new_r(Nor, word),
            (0, 0x00, ..) => DecodedInstruction::new_r(Sll, word),
            (0, 0x02, ..) => DecodedInstruction::new_r(Srl, word),
            (0, 0x2A, ..) => DecodedInstruction::new_r(Slt, word),
            (0, 0x2B, ..) => DecodedInstruction::new_r(Sltu, word),
            (0, 0x08, ..) => DecodedInstruction::new_r(Jr, word),
            (0, 0x09, ..) => DecodedInstruction::new_r(Jalr, word),
            (0, 0x0C, ..) => DecodedInstruction::new_r(Syscall, word),
            (0, 0x0D, ..) => DecodedInstruction::new_r(Brk, word),
            (0x11, 0x6, 0x10) => DecodedInstruction::new_f(Movs, word),
            (0x11, 0x6, 0x11) => DecodedInstruction::new_f(Movd, word),
            (0x11, 0x0, 0x10) => DecodedInstruction::new_f(Adds, word),
            (0x11, 0x0, 0x11) => DecodedInstruction::new_f(Addd, word),
            (0x11, 0x2, 0x10) => DecodedInstruction::new_f(Muls, word),
            (0x11, 0x2, 0x11) => DecodedInstruction::new_f(Muld, word),
            (0x11, 0x3, 0x10) => DecodedInstruction::new_f(Divs, word),
            (0x11, 0x3, 0x11) => DecodedInstruction::new_f(Divd, word),
            (0x11, 0x20, 0x11) => DecodedInstruction::new_f(Cvtsd, word),
            (0x11, 0x21, 0x14) => DecodedInstruction::new_f(Cvtdw, word),
            (0x11, .., 0x0) => DecodedInstruction::new_f(Mfc1, word),
            (0x11, .., 0x4) => DecodedInstruction::new_f(Mtc1, word),
            (0x08, ..) => DecodedInstruction::new_i(Addi, word),
            (0x09, ..) => DecodedInstruction::new_i(Addiu, word),
            (0x0A, ..) => DecodedInstruction::new_i(Slti, word),
            (0x0B, ..) => DecodedInstruction::new_i(Sltiu, word),
            (0x0C, ..) => DecodedInstruction::new_i(Andi, word),
            (0x0D, ..) => DecodedInstruction::new_i(Ori, word),
            (0x04, ..) => DecodedInstruction::new_i(Beq, word),
            (0x05, ..) => DecodedInstruction::new_i(Bne, word),
            (0x06, ..) => DecodedInstruction::new_i(Blez, word),
            (0x0F, ..) => DecodedInstruction::new_i(Lui, word),
            (0x20, ..) => DecodedInstruction::new_i(Lb, word),
            (0x21, ..) => DecodedInstruction::new_i(Lh, word),
            (0x23, ..) => DecodedInstruction::new_i(Lw, word),
            (0x24, ..) => DecodedInstruction::new_i(Lbu, word),
            (0x25, ..) => DecodedInstruction::new_i(Lhu, word),
            (0x2B, ..) => DecodedInstruction::new_i(Sw, word),
            (0x28, ..) => DecodedInstruction::new_i(Sb, word),
            (0x29, ..) => DecodedInstruction::new_i(Sh, word),
            (0x31, ..) => DecodedInstruction::new_i(Lwc1, word),
            (0x35, ..) => DecodedInstruction::new_i(Ldc1, word),
            (0x2, ..) => DecodedInstruction::new_j(J, word as i32),
            (0x3, ..) => DecodedInstruction::new_j(Jal, word as i32),
            (..) => DecodedInstruction::new_e(Unknown, word),
        }
    }
}

impl Default for DecodedInstruction {
    fn default() -> Self {
        use EOpType::*;
        DecodedInstruction::new_e(Unknown, 0)
    }
}
