fn main() {
    println!("hello world");
    /*
     * initialize code snippet
     * read through the code, splitting through InstructionExpressions basically.
     * then you have an evaluator per instruction type, that updates the data structure and emits the new snapshot
     * its almost like the lexer needs to run per instruction line. but that's terrible. why? we can be dynamic.
     * we can basically say:
     * when getting an instruction, we execute it based on the instruction
     * when getting a labelDef, just save its def into a map of "functions and their code"
     */
}
