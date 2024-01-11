use pest::Parser;
use pest_derive::Parser;
use std::fs;

use quill::parser::*;
use quill::ast::ASTNode;

fn main() {
    // For when I build the CLI Application, check for .ql files, not .quill files (too tedious)
    
    let raw_file: String = std::fs::read_to_string("src/test.quill").expect("can't read quill file");
    let ast = parse(&raw_file).expect("failed parse");

    println!("---");
}
