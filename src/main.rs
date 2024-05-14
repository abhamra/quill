use quill::optimizations::*;
use quill::parser::*;
use quill::type_checker::*;

use quill::ast::ASTNode;

fn main() {
    // TODO: Make a CLI interface for running Quill with .ql files
    // Files I've tested: test.quill, assignments.quill
    let raw_file: String =
        std::fs::read_to_string("src/test.quill").expect("can't read quill file");
    let ast = parse(&raw_file).expect("failed parse");
    // println!("{:?} \n -- \n", ast);

    // NOTE: The threshold is to check if we use the optimizations!
    let threshold_passed = respect_ratio(&ast);
    println!("{}", threshold_passed);

    type_check(&ast);

    // println!("{:?}", ast); // this works now
    ASTNode::print_nodes(&ast, 0);

    println!("---");
}
