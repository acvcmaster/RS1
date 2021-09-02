use crate::{cpu::Cpu, generic_error::GenericError};

impl Cpu {
    pub fn lui(&mut self, rt: u32, imm: u32, print: bool) -> Result<(), GenericError> {
        self.set_reg(rt, imm << 16);

        if print {
            println!("lui ${}, 0x{:x}", rt, imm);
        }

        Ok(())
    }

    pub fn ori(&mut self, rt: u32, rs: u32, imm: u32, print: bool) -> Result<(), GenericError> {
        self.set_reg(rt, self.reg(rs) | imm);

        if print {
            println!("ori ${}, ${}, 0x{:x}", rt, rs, imm);
        }

        Ok(())
    }

    pub fn sw(&mut self, rs: u32, rt: u32, imm: u32, print: bool) -> Result<(), GenericError> {
        let base = self.reg(rs);
        let value = self.reg(rt);
        let target = base.wrapping_add(imm);

        self.store32(target, value);

        if print {
            println!("sw ${}, 0x{:x}(${})", rt, imm, rs);
        }

        Ok(())
    }

    pub fn nop(&self, print: bool) -> Result<(), GenericError> {
        if print {
            println!("nop");
        }
        Ok(())
    }

    pub fn addiu(&mut self, rt: u32, rs: u32, imm: u32, print: bool) -> Result<(), GenericError> {
        self.set_reg(rt, self.reg(rs).wrapping_add(imm));

        if print {
            println!("addiu ${}, ${}, 0x{:x}", rt, rs, imm);
        }

        Ok(())
    }

    pub fn sll(&mut self, rt: u32, rd: u32, shamt: u32, print: bool) -> Result<(), GenericError> {
        self.set_reg(rd, self.reg(rt) << shamt);

        if print {
            println!("sll ${}, ${}, 0x{:x}", rd, rt, shamt);
        }

        Ok(())
    }

    pub fn srl(&mut self, rt: u32, rd: u32, shamt: u32, print: bool) -> Result<(), GenericError> {
        self.set_reg(rd, self.reg(rt) >> shamt);

        if print {
            println!("srl ${}, ${}, 0x{:x}", rd, rt, shamt);
        }

        Ok(())
    }

    pub fn j(&mut self, addr: u32, print: bool) -> Result<(), GenericError> {
        if print {
            println!("j 0x{:x}", addr);
        }

        // Execute instruction at the branch delay slot
        let result = self.decode_and_execute(self.branch_delay_slot, print);

        if result.is_err() {
            return result;
        } else {
            self.pc = (self.pc & 0xf0000000) | (addr << 2);
            Ok(())
        }
    }

    pub fn or(&mut self, rs: u32, rt: u32, rd: u32, print: bool) -> Result<(), GenericError> {
        self.set_reg(rd, self.reg(rs) | self.reg(rt));

        if print {
            println!("or ${}, ${}, ${}", rd, rs, rt);
        }

        Ok(())
    }

    pub fn mtc0(&mut self, rt: u32, rs: u32, print: bool) -> Result<(), GenericError> {
        match rs {
            12 => {
                self.sr = self.reg(rt);

                if print {
                    println!("mtc0 ${}, ${}", rt, rs);
                }

                Ok(())
            }
            _ => Err(GenericError {
                message: format!("UNHANDLED_COP0_REGISTER (${})", rs),
            }),
        }
    }

    fn relative_branch(&mut self, offset: u32, print: bool) -> Result<(), GenericError> {
        let result = self.decode_and_execute(self.branch_delay_slot, print);

        if result.is_err() {
            return result;
        }

        self.pc = self.pc.wrapping_add(offset << 2);
        Ok(())
    }

    pub fn bne(&mut self, rs: u32, rt: u32, imm: u32, print: bool) -> Result<(), GenericError> {
        if print {
            println!("bne ${}, ${}, 0x{:x}", rs, rt, imm)
        }

        if self.reg(rs) == self.reg(rt) {
            let result = self.relative_branch(imm, print);
            if result.is_err() {
                return result;
            }
        }

        Ok(())
    }

    pub fn addi(&mut self, rt: u32, rs: u32, imm: u32, print: bool) -> Result<(), GenericError> {
        if let Some(sum) = self.reg(rs).checked_add(imm) {
            self.set_reg(rt, sum);

            if print {
                println!("addi ${}, ${}, 0x{:x}", rt, rs, imm);
            }

            Ok(())
        } else {
            return Err(GenericError {
                message: format!("ADDI_ARITHMETIC_OVERFLOW"),
            });
        }
    }
}
