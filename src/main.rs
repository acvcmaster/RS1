use cpu::Cpu;

use bios::Bios;
use logger::handle_critical_result;

pub mod bios;
mod cpu;
pub mod logger;
mod memory;

/// The entry point of the program
fn main() {
    let result = Bios::new(&"bios/scph1001.bin".to_string());
    let bios = handle_critical_result(result, Some("Failed to load bios:".to_string()));
    let mut core = Cpu::new();

    core.load_bios(bios);

    
}
