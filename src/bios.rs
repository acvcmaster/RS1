use std::{
    fmt::Debug,
    fs::File,
    io::{self, Error, ErrorKind, Read},
    path::Path,
};

/// 512 KiB
const BIOS_SIZE: usize = 524288;

/// BIOS image
#[derive(Clone)]
pub struct Bios {
    /// Memory data
    pub data: Vec<u8>,
}

impl Bios {
    pub fn new(path: &String) -> Result<Self, io::Error> {
        let file = File::open(Path::new(path))?;
        let mut buffer: Vec<u8> = Vec::new();

        file.take(BIOS_SIZE as u64).read_to_end(&mut buffer)?;

        if buffer.len() == BIOS_SIZE {
            Ok(Self { data: buffer })
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "INVALID_BIOS_SIZE"))
        }
    }

    pub fn load32(&self, offset: u32) -> u32 {
        let offset = offset as usize;
        let b0 = self.data[offset + 0] as u32;
        let b1 = self.data[offset + 1] as u32;
        let b2 = self.data[offset + 2] as u32;
        let b3 = self.data[offset + 3] as u32;
        b0 | (b1 << 8) | (b2 << 16) | (b3 << 24)
    }
}

impl Default for Bios {
    fn default() -> Self {
        Bios { data: Vec::new() }
    }
}

impl Debug for Bios {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.data.len();

        write!(
            f,
            "Bios {{ data<{}>: [{}] }}",
            size,
            if size < 1 { "" } else { "..." }
        )
    }
}
