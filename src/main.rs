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
    let buff = r"+++++ +++++ [         Inicia as células com os valores:
  > +++++ +++         80
  > +++++ +++++ +     110
  > +++++ +++++       100
  > ++++              40
  > +++               30
  > +++++ +++         80
  > +++++ +++++ ++    120
  > +++++ +++++ +     110
  > +++++ +++++       100
  > +++++ +++++ +     110
  > +++               30
  > +                 10
  < <<<<< <<<<< < -
]
> - .                 Escreve 'O'
> -- .                Escreve 'l'
> ---.                Escreve 'a'
> ++++ .              Escreve vírgula
> ++ .                Escreve ' '
> --- .               Escreve 'M'
> --- .               Escreve 'u'
> .                   Escreve 'n'
> .                   Escreve 'd'
> + .                 Escreve 'o'
> +++ .               Escreve '!'
> .                   Escreve nova linha";

    let mut instructions: Vec<Instruction> = Vec::new();
    let mut lexer = Lexer::new(buff.chars().rev().collect());

    let mut loop_stack: Vec<usize> = Vec::new();

    while let Some(c) = lexer.next() {
        match c {
            '+' => {
                let mut count = 1;

                loop {
                    count += 1;

                    match lexer.next() {
                        Some('+') => continue,
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
                    count += 1;
                    match lexer.next() {
                        Some('-') => continue,
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
                    count += 1;
                    match lexer.next() {
                        Some('>') => continue,
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
                    count += 1;
                    match lexer.next() {
                        Some('<') => continue,
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
                let pos = loop_stack.pop().unwrap();
                instructions[pos] = Instruction::StartLoop(Some(instructions.len()));
                instructions.push(Instruction::EndLoop(pos));
            }
            _ => {}
        }
    }
}
