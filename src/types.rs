#[derive(Copy, Clone)]
pub enum ByteCode {
    LoadVal(i64),
    WriteVar(char),
    ReadVar(char),
    Add,
    Subtract,
    Multiply,
	Divide,
    Return,
}

#[derive(Copy, Clone, Debug)]
pub struct Variable {
    pub name: Option<char>,
    pub value: i64,
}

#[derive(Clone)]
pub struct Program {
    pub bytecode_vec: Vec<ByteCode>,
    pub var_stack: Vec<Variable>
}

#[derive(Debug)]
pub enum ProgramError {
    StackUnderflow,
}
