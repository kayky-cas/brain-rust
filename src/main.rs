use std::{
    env::args,
    fs::File,
    io::{self, stdin, stdout},
    process::exit,
};

use brain_rust::{interactive::Interactive, program::Program};

struct BrainRust;

impl BrainRust {
    fn usage(program_name: String) -> io::Result<()> {
        eprintln!("Usage: {} [filename | -i]", program_name);
        exit(1);
    }

    fn interactive() -> io::Result<()> {
        let program = Program::new(Vec::new());
        let mut interactive = Interactive::new(program);

        interactive.run()
    }

    fn run(file_name: &str) -> io::Result<()> {
        let file = File::open(file_name).unwrap();
        let mut program = Program::from(file);

        let mut stdout = stdout();
        let stdin = stdin();

        program.run(&mut stdin.lock(), &mut stdout)
    }
}

fn main() -> io::Result<()> {
    let mut args = args();

    let program_name = args.next().unwrap();

    match args.next().as_deref() {
        Some("-i") => BrainRust::interactive(),
        Some(file_name) => BrainRust::run(file_name),
        _ => BrainRust::usage(program_name),
    }
}
