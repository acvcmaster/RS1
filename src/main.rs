use cpu::Cpu;
use emulator_args::parse_emulator_args;

use bios::Bios;
use logger::handle_critical_result;

pub mod bios;
mod cpu;
mod cpu_instructions;
mod decoded_instruction;
mod emulator_args;
pub mod generic_error;
pub mod logger;
mod memory;
mod memory_region;

/// The entry point of the program
fn main() {
    let args = parse_emulator_args();

    let result = Bios::new(&args.bios);
    let bios = handle_critical_result(result, Some("Failed to load bios:"));

    let mut cpu = Cpu::new();

    cpu.load_bios(bios);

    loop {
        cpu.run_next_instruction(args.debug);
    }
}
