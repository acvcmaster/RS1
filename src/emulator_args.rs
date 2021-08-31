use clap::{App, Arg};

#[derive(Debug, Clone, Default)]
pub struct EmulatorArgs {
    pub bios: String,
}

pub fn parse_emulator_args() -> EmulatorArgs {
    let matches = App::new("Rust Station 1")
        .version("v0.1")
        .author("acvcmaster <acvcnos@hotmail.com>")
        .about("Simple PS1 emulator written in Rust")
        .arg(
            Arg::new("bios")
                .short('b')
                .long("bios")
                .value_name("BIOS")
                .about("Sets the path of the bios file to use")
                .required(true),
        )
        .get_matches();

    EmulatorArgs {
        bios: matches.value_of("bios").unwrap_or_default().to_owned(),
    }
}
