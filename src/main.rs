use std::{
    env::args,
    fs::File,
    io::{self, stdin, stdout, Read, Write},
    process::exit,
};

use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal::{self, WindowSize},
};

#[derive(Debug)]
enum Instruction {
    Increment(usize),
    Decrement(usize),
    ShiftLeft(usize),
    ShiftRight(usize),
    Output,
    Input,
    StartLoop(Option<usize>),
    EndLoop(usize),
}

struct Lexer<'a> {
    buffer: &'a [u8],
    cursor: usize,
}

impl<'a> Lexer<'a> {
    fn new(buffer: &'a [u8]) -> Self {
        Lexer { buffer, cursor: 0 }
    }

    fn next_char(&mut self) -> Option<char> {
        while self.cursor < self.buffer.len() {
            let c = self.buffer[self.cursor];

            self.cursor += 1;

            if matches!(c, b'>' | b'<' | b'+' | b'-' | b'.' | b',' | b'[' | b']') {
                return Some(c as char);
            }
        }

        None
    }

    fn back(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }
}

impl Iterator for Lexer<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_char()
    }
}

struct Parser;

impl Parser {
    fn parse(mut lexer: Lexer) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut loop_stack: Vec<usize> = Vec::new();

        while let Some(c) = lexer.next() {
            match c {
                c @ '<' | c @ '>' | c @ '+' | c @ '-' => {
                    let mut count = 1;
                    loop {
                        match lexer.next() {
                            Some(ch) if ch == c => count += 1,
                            Some(_) => {
                                lexer.back();
                                break;
                            }
                            None => break,
                        }
                    }

                    match c {
                        '>' => instructions.push(Instruction::ShiftRight(count)),
                        '<' => instructions.push(Instruction::ShiftLeft(count)),
                        '+' => instructions.push(Instruction::Increment(count)),
                        '-' => instructions.push(Instruction::Decrement(count)),
                        _ => {}
                    }
                }
                '.' => instructions.push(Instruction::Output),
                ',' => instructions.push(Instruction::Input),
                '[' => {
                    let pos = instructions.len();
                    instructions.push(Instruction::StartLoop(None));
                    loop_stack.push(pos);
                }
                ']' => {
                    let pos = loop_stack.pop().expect("Invalid loop");

                    instructions[pos] = Instruction::StartLoop(Some(instructions.len()));
                    instructions.push(Instruction::EndLoop(pos));
                }
                _ => {}
            }
        }

        instructions
    }
}

struct Program {
    instructions: Vec<Instruction>,
    cells: Vec<u8>,
    pointer: usize,
    instruction_pointer: usize,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Self {
        Program {
            instructions,
            cells: vec![0; 5],
            pointer: 0,
            instruction_pointer: 0,
        }
    }

    fn run(&mut self, stdin: &mut impl Read, stdout: &mut impl Write) {
        while self.instruction_pointer < self.instructions.len() {
            match self.instructions[self.instruction_pointer] {
                Instruction::Increment(by) => {
                    self.cells[self.pointer] =
                        ((self.cells[self.pointer] as usize + by) % 255) as u8;
                }
                Instruction::Decrement(by) => {
                    self.cells[self.pointer] =
                        ((self.cells[self.pointer] as usize - by) % 255) as u8;
                }
                Instruction::ShiftLeft(by) => {
                    if self.pointer < by {
                        self.pointer = 0;
                    } else {
                        self.pointer -= by;
                    }
                }
                Instruction::ShiftRight(by) => {
                    if self.pointer + by >= self.cells.len() {
                        self.cells.resize(self.pointer + by + 1, 0);
                    }

                    self.pointer += by;
                }
                Instruction::Output => print!("{}", self.cells[self.pointer] as char),
                Instruction::Input => {
                    let mut buf = [0; 1];

                    stdin.read_exact(&mut buf).unwrap();
                    stdout.flush().unwrap();

                    self.cells[self.pointer] = buf[0];
                }
                Instruction::StartLoop(Some(end_loop_pos)) => {
                    if self.cells[self.pointer] == 0 && end_loop_pos < self.cells.len() - 1 {
                        self.instruction_pointer = end_loop_pos + 1;
                    }
                }
                Instruction::EndLoop(start_loop_pos) => {
                    if self.cells[self.pointer] != 0 {
                        self.instruction_pointer = start_loop_pos;
                    }
                }
                _ => {}
            };

            self.instruction_pointer += 1;
        }
    }

    fn interactive_run(&mut self) -> io::Result<()> {
        let mut stdout = stdout();
        let _stdin = stdin();

        execute!(stdout, cursor::Hide)?;

        let WindowSize { rows, columns, .. } = terminal::window_size()?;

        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

        let pos = term::Vec2::new(0, 0);
        let size = term::Vec2::new(columns, rows);

        // Borders
        term::write_box(
            &mut stdout,
            style::Color::White,
            pos,
            size,
            Some("Brain Rust"),
        )?;

        let inner_rows = rows - 2;
        let inner_columns = columns - 2;

        // Buffer box

        let mut buffer_text = String::new();

        for (idx, &cell) in self.cells.iter().enumerate() {
            buffer_text.push_str(&format!("{}", cell));
            if idx < self.cells.len() - 1 {
                buffer_text.push_str(" | ");
            }
        }

        term::write_box(
            &mut stdout,
            style::Color::Red,
            term::Vec2::new(3, 3),
            term::Vec2::new((buffer_text.len() + 4) as u16, 3),
            Some("Buffer"),
        )?;

        queue!(
            stdout,
            cursor::MoveTo(5, 4),
            style::PrintStyledContent(buffer_text.with(style::Color::Red))
        )?;

        stdout.flush()?;

        loop {}

        Ok(())
    }
}

mod term {
    use std::io;
    use std::io::Stdout;

    use crossterm::{
        cursor, queue,
        style::{self, Stylize},
    };

    pub struct Vec2 {
        pub x: u16,
        pub y: u16,
    }

    impl Vec2 {
        pub fn new(x: u16, y: u16) -> Self {
            Vec2 { x, y }
        }
    }

    pub fn write_box(
        stdout: &mut Stdout,
        color: style::Color,
        pos: Vec2,
        size: Vec2,
        title: Option<&str>,
    ) -> io::Result<()> {
        for y in 0..size.y {
            for x in 0..size.x {
                if (y == 0 || y == size.y - 1) || (x == 0 || x == size.x - 1) {
                    queue!(
                        stdout,
                        cursor::MoveTo(pos.x + x, pos.y + y),
                        style::PrintStyledContent("█".with(color))
                    )?;
                }
            }
        }

        if let Some(title) = title {
            let text_padding = (size.x as f32 * 0.05f32) as u16;

            queue!(
                stdout,
                cursor::MoveTo(text_padding + pos.x, pos.y),
                style::PrintStyledContent(title.with(color))
            )?;
        }

        Ok(())
    }
}

fn main() {
    let mut args = args();

    let program_name = args.next().unwrap();

    match args.next().as_deref() {
        Some("-i") => {
            let mut program = Program::new(Vec::new());
            program.interactive_run();
        }

        Some(file_name) => {
            let mut file = File::open(file_name).unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            let lexer = Lexer::new(&buf);
            let instructions = Parser::parse(lexer);
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
}
