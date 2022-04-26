use std::result::Result;
use crate::types::*;

macro_rules! execute {
    ($code:expr, $op:tt) => {{
        if let Some(a) = $code.var_stack.pop() {
            if let Some(b) = $code.var_stack.pop() {
                $code.var_stack.push(Variable {
                    name: None,
                    value: (b.value $op a.value),
                });
                None
            } else { Some(ProgramError::StackUnderflow) }
        } else { Some(ProgramError::StackUnderflow) }
    }
}}

pub fn interpret(bytecode_vec: Vec<ByteCode>) -> Result<Variable, ProgramError> {
    let mut code = Program {
        bytecode_vec,
        var_stack: Vec::new(),
    };

	// iterate and match the bytecode vector
    for command in code.bytecode_vec {
        if let Some(err) = match command {
            ByteCode::LoadVal(value) => {
                code.var_stack.push(Variable {
                    name: None,
                    value,
                });
                None
            },
            ByteCode::WriteVar(c) => {
                let loaded_value = code.var_stack.pop();
                if let Some(v) = loaded_value {
                    code.var_stack.push(Variable {
                        name: Some(c),
                        value: v.value,
                    })
                }
                None
            },
            ByteCode::ReadVar(c) => {

                let read_value = code.var_stack.iter().find(|&&x| x.name == Some(c));
                if let Some(v) = read_value {
                    let var = v.clone();
                    code.var_stack.push(var.clone())
                }
                None
            },
            ByteCode::Multiply => execute!(code, *),
			ByteCode::Divide => execute!(code, /),
            ByteCode::Add => execute!(code, +),
			ByteCode::Subtract => execute!(code, -),
            ByteCode::Return => break,
        } {
            return Err(err);
        }
    }

    if let Some(v) = code.var_stack.pop() {
        Ok(v)
    } else {
        Err(ProgramError::StackUnderflow)
    }
}
