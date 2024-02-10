#[derive(Debug)]
pub enum Instruction {
    Increment(usize),
    Decrement(usize),
    ShiftLeft(usize),
    ShiftRight(usize),
    Output,
    Input,
    StartLoop(Option<usize>),
    EndLoop(usize),
    Command(String),
}
