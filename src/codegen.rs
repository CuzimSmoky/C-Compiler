use crate::ast::*;
use crate::asm_ast::*;


pub fn run(program: Program) {
    let asm = generate_program(program);
    println!("ASM AST: {:?}", asm);
}


pub fn generate_program(program: Program) -> AsmProgram{
    let asm_function: AsmFunction = generate_function(program.function);
    return AsmProgram {
        asm_function: asm_function,
    }
}
pub fn generate_function(function: Function) -> AsmFunction{
    let instructions: Vec<AsmInstruction> = generate_instruction(function.body);
    let asm_function: AsmFunction = AsmFunction {
         name: function.name ,
         body: instructions,
        };
    return asm_function;

}
pub fn generate_instruction(statement: Statement) -> Vec<AsmInstruction>{
    if let Statement::Return(expression) = statement {
        let operand = generate_immideate(expression);
        let register = Register::Rax;
        let mov_instruction = AsmInstruction::Mov{
            src: operand,
            dst: register,
        };
        let return_instruction = AsmInstruction::Ret;

        let mut instructions: Vec<AsmInstruction> = Vec::new();
        instructions.push(mov_instruction);
        instructions.push(return_instruction);
        return instructions;
    } else {
        panic!("Expected statement but found {:?}", statement);
    }
}
pub fn generate_immideate(expression: Expression) -> Operand{
    if let Expression::Constant(value) = expression {
        return Operand::Imm(value);
    } else {
        panic!("Expected constant but found {:?}", expression);
    }
}
