use super::Instruction;

#[derive(Debug)]
pub struct Function {
    pub instructions: Vec<Instruction>,
    pub labels: Vec<(String, usize)>,
}

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}
