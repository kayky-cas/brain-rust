use std::{
    env::{args, Args},
    fs::File,
    io::{stdin, stdout, Read, Write},
    process::exit,
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
                '+' => {
                    let mut count = 1;

                    loop {
                        match lexer.next() {
                            Some('+') => count += 1,
                            Some(_) => {
                                lexer.back();
                                break;
                            }
                            None => break,
                        }
                    }

                    instructions.push(Instruction::Increment(count));
                }
                '-' => {
                    let mut count = 1;
                    loop {
                        match lexer.next() {
                            Some('-') => count += 1,
                            Some(_) => {
                                lexer.back();
                                break;
                            }
                            None => break,
                        }
                    }
                    instructions.push(Instruction::Decrement(count));
                }
                '>' => {
                    let mut count = 1;
                    loop {
                        match lexer.next() {
                            Some('>') => count += 1,
                            Some(_) => {
                                lexer.back();
                                break;
                            }
                            None => break,
                        }
                    }
                    instructions.push(Instruction::ShiftRight(count));
                }
                '<' => {
                    let mut count = 1;
                    loop {
                        match lexer.next() {
                            Some('<') => count += 1,
                            Some(_) => {
                                lexer.back();
                                break;
                            }
                            None => break,
                        }
                    }
                    instructions.push(Instruction::ShiftLeft(count));
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
            cells: vec![0],
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
}

fn interface(mut args: Args) -> Vec<u8> {
    let program_name = args.next().unwrap();

    if let Some(file_name) = args.next() {
        let mut file = File::open(file_name).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        buf
    } else {
        eprintln!("Usage: {} [filename]", program_name);
        exit(1);
    }
}

fn main() {
    let buf = interface(args());

    let lexer = Lexer::new(&buf);
    let instructions = Parser::parse(lexer);
    let mut program = Program::new(instructions);

    let mut stdout = stdout();
    let stdin = stdin();

    program.run(&mut stdin.lock(), &mut stdout);
}
