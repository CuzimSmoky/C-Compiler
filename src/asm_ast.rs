use crate::ast::*;

#[derive(Debug)]
pub struct AsmProgram {
    pub asm_function: AsmFunction,
}
#[derive(Debug)]
pub struct AsmFunction {
    pub name: Identifier,
    pub body: Vec<AsmInstruction>,
}
#[derive(Debug)]
pub enum AsmInstruction {
    Mov {
        src: Operand,
        dst: Register,
    },
    Ret,
}

#[derive(Debug)]
pub enum Operand {
    Imm(i64),
    Reg(Register),
}

#[derive(Debug)]
pub enum Register {
    Rax,
    Rbx,
    Rcx,
    Rdx,
}
