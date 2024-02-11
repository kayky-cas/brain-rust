mod instruction;
mod interactive;
mod parser;
mod program;
mod term;

use std::{
    fs::File,
    io::{self, stdin, stdout},
    process::exit,
};

use crate::{interactive::Interactive, program::Program};

pub struct BrainRust;

impl BrainRust {
    pub fn usage(program_name: String) -> io::Result<()> {
        eprintln!("Usage: {} [filename | -i]", program_name);
        exit(1);
    }

    pub fn interactive() -> io::Result<()> {
        let program = Program::new(Vec::new());
        let mut interactive = Interactive::new(program);

        interactive.run()
    }

    pub fn run(file_name: &str) -> io::Result<()> {
        let file = File::open(file_name).unwrap();
        let mut program = Program::from(file);

        let mut stdout = stdout();
        let stdin = stdin();

        program.run(&mut stdin.lock(), &mut stdout)
    }
}
