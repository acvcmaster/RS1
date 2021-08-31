use crate::{cpu::Cpu, generic_error::GenericError, logger::handle_critical_result};

impl Cpu {
    pub fn lui(&mut self, rt: u32, imm: u32, print: bool) -> Result<(), GenericError> {
        self.gpr[rt as usize] = imm << 16;

        if print {
            println!("lui ${}, 0x{:x}", rt, imm);
        }

        Ok(())
    }

    pub fn ori(&mut self, rt: u32, rs: u32, imm: u32, print: bool) -> Result<(), GenericError> {
        self.gpr[rt as usize] = self.gpr[rs as usize] | imm;

        if print {
            println!("ori ${}, ${}, 0x{:x}", rt, rs, imm);
        }

        Ok(())
    }

    pub fn sw(&mut self, rt: u32, rs: u32, imm: u32, print: bool) -> Result<(), GenericError> {
        let base = self.gpr[rs as usize] as i32;
        let offset = imm as i32;

        let result = self.memory.store32((base + offset) as u32, imm);

        if result.is_err() {
            return result;
        } else {
            if print {
                println!("sw ${}, {}(${})", rt, offset, rs);
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
        self.gpr[rt as usize] = self.gpr[rs as usize] + imm;

        if print {
            println!("addiu ${}, ${}, 0x{:x}", rt, rs, imm);
        }

        Ok(())
    }
}
