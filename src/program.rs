use std::{
    fs::File,
    io::{Read, Write},
};

use crate::parser::{Lexer, Parser, ParserMode};

use super::instruction::Instruction;

pub struct Program {
    instructions: Vec<Instruction>,
    cells: Vec<u8>,
    pointer: usize,
    instruction_pointer: usize,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Program {
            instructions,
            cells: vec![0; 5],
            pointer: 0,
            instruction_pointer: 0,
        }
    }

    pub fn run(&mut self, stdin: &mut impl Read, stdout: &mut impl Write) {
        while self.instruction_pointer < self.instructions.len() {
            match self.instructions[self.instruction_pointer] {
                Instruction::Increment(by) => {
                    let wrapping = self.cells[self.pointer].wrapping_add((by % 255) as u8);
                    self.cells[self.pointer] = wrapping;
                }
                Instruction::Decrement(by) => {
                    let wrapping = self.cells[self.pointer].wrapping_sub((by % 255) as u8);
                    self.cells[self.pointer] = wrapping;
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
                Instruction::Output => {
                    let _ = write!(stdout, "{}", self.cells[self.pointer] as char);
                }
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
                Instruction::Command(ref cmd) => {
                    let mut cmd_spl = cmd.split_whitespace();

                    let command = cmd_spl.next().unwrap().to_lowercase();
                    let args: Vec<&str> = cmd_spl.collect();

                    match command.as_str() {
                        "include" => {
                            let files: Vec<_> =
                                args.iter().map(|x| File::open(x).unwrap()).collect();

                            for mut file in files {
                                let mut buf = Vec::new();
                                let _ = file.read_to_end(&mut buf);

                                let lexer = Lexer::new(&buf);

                                let mut parser = Parser::parse(lexer, ParserMode::Normal);

                                self.instructions.append(&mut parser);
                            }
                        }
                        _ => todo!("{}: {}, {:?}", cmd, command, args),
                    }
                }
                _ => {}
            };

            self.instruction_pointer += 1;
        }
    }

    pub fn append_instructions(&mut self, instructions: &mut Vec<Instruction>) {
        self.instructions.append(instructions);
    }

    pub fn cells(&self) -> &[u8] {
        &self.cells
    }

    pub fn pointer(&self) -> usize {
        self.pointer
    }
}
