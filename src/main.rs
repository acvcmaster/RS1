use cpu::Cpu;

use bios::Bios;
use logger::handle_critical_result;

pub mod bios;
mod cpu;
pub mod logger;
mod memory;
mod memory_region;
pub mod generic_error;

/// The entry point of the program
fn main() {
    let result = Bios::new(&"bios/scph1001.bin".to_string());
    let bios = handle_critical_result(result, Some("Failed to load bios:".to_string()));
    let mut cpu = Cpu::new();

    cpu.load_bios(bios);

    loop {
        cpu.run_next_instruction();
    }
}
