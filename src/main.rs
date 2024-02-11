use std::{
    env::args,
    fs::File,
    io::{self, stdin, stdout},
    process::exit,
};

use brain_rust::{interactive::Interactive, program::Program};

fn main() -> io::Result<()> {
    let mut args = args();

    let program_name = args.next().unwrap();

    match args.next().as_deref() {
        Some("-i") => {
            let program = Program::new(Vec::new());
            let mut interactive = Interactive::new(program);

            interactive.run()?;
        }

        Some(file_name) => {
            let file = File::open(file_name).unwrap();
            let mut program = Program::from(file);

            let mut stdout = stdout();
            let stdin = stdin();

            program.run(&mut stdin.lock(), &mut stdout);
        }
        _ => {
            eprintln!("Usage: {} [filename | -i]", program_name);
            exit(1);
        }
    }

    Ok(())
}
