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

    pub fn sw(&mut self, rt: u32, rs: u32, imm: u32, print: bool) -> Result<(), GenericError> {
        let base = self.gpr[rs as usize] as i32;
        let offset = imm as i32;
        let value = self.gpr[rt as usize];

        let result = self.memory.store32((base + offset) as u32, value);

        if result.is_err() {
            return result;
        } else {
            if print {
                println!("sw ${}, 0x{:x}(${})", rt, offset, rs);
            }
            Ok(())
        }
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
}
