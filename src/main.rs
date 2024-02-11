use std::{env::args, io};

use brain_rust::BrainRust;

fn main() -> io::Result<()> {
    let mut args = args();

    let program_name = args.next().ok_or(io::ErrorKind::InvalidInput)?;

    match args.next().as_deref() {
        Some("-i") => BrainRust::interactive(),
        Some(file_name) => BrainRust::run(file_name),
        _ => BrainRust::usage(program_name),
    }
}
