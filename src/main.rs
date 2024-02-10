use std::{
    env::args,
    fs::File,
    io::{self, stdin, stdout, Read},
    process::exit,
};

use brain_rust::{
    interactive::Interactive,
    parser::{Lexer, Parser, ParserMode},
    program::Program,
};

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
            let mut file = File::open(file_name).unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            let lexer = Lexer::new(&buf);
            let instructions = Parser::parse(lexer, ParserMode::Normal);
            let mut program = Program::new(instructions);

            let mut stdout = stdout();
            let stdin = stdin();

            program.run(&mut stdin.lock(), &mut stdout);
        }
        _ => {
            eprintln!("Usage: {} [filename]", program_name);
            exit(1);
        }
    }

    Ok(())
}
