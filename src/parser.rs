use super::instruction::Instruction;

pub struct Lexer<'a> {
    buffer: &'a [u8],
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Lexer { buffer, cursor: 0 }
    }

    fn next_token(&mut self) -> Option<char> {
        while self.cursor < self.buffer.len() {
            let c = self.buffer[self.cursor];

            self.cursor += 1;

            if matches!(
                c,
                b'>' | b'<' | b'+' | b'-' | b'.' | b',' | b'[' | b']' | b'#'
            ) {
                return Some(c as char);
            }
        }

        None
    }

    fn next_char(&mut self) -> Option<char> {
        self.buffer.get(self.cursor).map(|&c| {
            self.cursor += 1;
            c as char
        })
    }

    fn back(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }
}

impl Iterator for Lexer<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

pub struct Parser;

pub enum ParserMode {
    Normal,
    Command,
}

impl Parser {
    pub fn parse(mut lexer: Lexer, mode: ParserMode) -> Vec<Instruction> {
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
                '#' if matches!(mode, ParserMode::Command) => {
                    let mut command = String::new();

                    while let Some(c) = lexer.next_char() {
                        if c == ';' {
                            break;
                        }
                        command.push(c);
                    }

                    println!("Command: {}", command);

                    instructions.push(Instruction::Command(command));
                }
                _ => {}
            }
        }

        instructions
    }
}
