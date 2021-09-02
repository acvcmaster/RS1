use crate::generic_error::GenericError;

pub struct Memlcontrol {}

impl Memlcontrol {
    pub fn store_32(offset: u32, word: u32) -> Option<Result<(), GenericError>> {
        match offset {
            0 => {
                if word != 0x1f000000 {
                    return Some(Err(GenericError {
                        message: format!("STORE32_BAD_EXPANSION_1_BASE_ADDRESS (0x{:x})", word),
                    }));
                }
            }
            4 => {
                if word != 0x1f802000 {
                    return Some(Err(GenericError {
                        message: format!("STORE32_BAD_EXPANSION_2_BASE_ADDRESS (0x{:x})", word),
                    }));
                }
            }
            _ => {
                return Some(Err(GenericError {
                    message: format!("STORE32_UNHANDLED_MEMLCONTROL_WRITE (0x{:x})", word),
                }));
            }
        }
        None
    }
    
    pub fn read_32(offset: u32) -> u32 {
        match offset {
            0 => 0x1f000000,
            4 => 0x1f802000,
            _ => 0,
        }
    }
}
