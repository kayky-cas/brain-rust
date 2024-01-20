#[derive(Debug, Clone, Copy)]
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

struct Lexer {
    buffer: Vec<char>,
}

impl Lexer {
    fn new(buffer: Vec<char>) -> Self {
        Lexer { buffer }
    }

    fn next_char(&mut self) -> Option<char> {
        loop {
            let c = self.buffer.pop()?;
            if matches!(c, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']') {
                return Some(c);
            }
        }
    }

    fn push(&mut self, c: char) {
        self.buffer.push(c);
    }
}

impl Iterator for Lexer {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_char()
    }
}

fn main() {
    let buff = r"++++++++++[>++++++++>+++++++++++>++
++++++++>++++>+++>++++++++>++++++++
++++>+++++++++++>++++++++++>+++++++
++++>+++>+<<<<<<<<<<<<-]>-.>--.>---
.>++++.>++.>---.>---.>.>.>+.>+++.>.";

    let mut instructions: Vec<Instruction> = Vec::new();
    let mut lexer = Lexer::new(buff.chars().rev().collect());

    let mut loop_stack: Vec<usize> = Vec::new();

    while let Some(c) = lexer.next() {
        match c {
            '+' => {
                let mut count = 1;

                loop {
                    match lexer.next() {
                        Some('+') => count += 1,
                        Some(c) => {
                            lexer.push(c);
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
                        Some(c) => {
                            lexer.push(c);
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
                        Some(c) => {
                            lexer.push(c);
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
                        Some(c) => {
                            lexer.push(c);
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

    let mut cells: Vec<u8> = vec![0];
    let mut pointer = 0;
    let mut instruction_pointer = 0;

    while instruction_pointer < instructions.len() {
        match instructions[instruction_pointer] {
            Instruction::Increment(by) => {
                // TODO: Handle usize -> u8
                cells[pointer] = cells[pointer].wrapping_add(by as u8);
            }
            Instruction::Decrement(by) => {
                // TODO: Handle usize -> u8
                cells[pointer] = cells[pointer].wrapping_sub(by as u8);
            }
            Instruction::ShiftLeft(by) => {
                if pointer < by {
                    pointer = 0;
                } else {
                    pointer -= by;
                }
            }
            Instruction::ShiftRight(by) => {
                if pointer + by >= cells.len() {
                    cells.resize(pointer + by + 1, 0);
                }

                pointer += by;
            }
            Instruction::Output => print!("{}", cells[pointer] as char),
            Instruction::Input => todo!(),
            Instruction::StartLoop(Some(end_loop_pos)) => {
                if cells[pointer] == 0 && end_loop_pos < cells.len() - 1 {
                    instruction_pointer = end_loop_pos + 1;
                }
            }
            Instruction::EndLoop(start_loop_pos) => {
                if cells[pointer] != 0 {
                    instruction_pointer = start_loop_pos;
                }
            }
            _ => {}
        };

        instruction_pointer += 1;
    }
}
