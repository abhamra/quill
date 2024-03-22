use quill::parser::*;
use quill::optimizations::*;
use quill::type_checker::*;

use quill::ast::{ASTNode, NodeKind, RespectExpr, GateExpr, OutputExpr, ValueExpr};

fn main() {
    // For when I build the CLI Application, check for .ql files, not .quill files (too tedious)
    // Files I've tested: test.quill, assignments.quill
    let raw_file: String = std::fs::read_to_string("src/test.quill").expect("can't read quill file");
    let ast = parse(&raw_file).expect("failed parse");
    // println!("{:?} \n -- \n", ast);

    let threshold_passed = respect_ratio(&ast);
    println!("{}", threshold_passed);
    
    type_check(&ast); 

    // println!("{:?}", ast); // this works now
    ASTNode::print_nodes(&ast, 0);

    // println!("{:?}", ast);

    println!("---");
}
