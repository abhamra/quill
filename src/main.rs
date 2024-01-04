use pest::Parser;
use pest_derive::Parser;
use std::fs;

use quill::parser::{QuillParser, Rule};

fn main() {
    // some tests
    
    let unparsed_quill = fs::read_to_string("src/test.quill").expect("Cannot read file");
    let program = QuillParser::parse(Rule::Program, &unparsed_quill)
        .expect("failed parse")
        .next().unwrap();

    println!("{:#}", program);

    println!("---");

    // NOTE TO SELF: MAKE FORMAL TESTS SOON FOR PARSER

    let qubit_parse = QuillParser::parse(Rule::Qubit, "00");
    println!("{:?}\n", qubit_parse);
    
    let qubit_parse2 = QuillParser::parse(Rule::Qubit, "0");
    println!("{:?}\n", qubit_parse2);

    let qreg_parse1 = QuillParser::parse(Rule::QReg, "0[5]");
    println!("{:?}\n", qreg_parse1);

    let qreg_parse2 = QuillParser::parse(Rule::QReg, "0[5] + 1[4]");
    println!("{:?}\n", qreg_parse2);

    let qreg_parse3 = QuillParser::parse(Rule::QReg, "0[var]");
    println!("{:?}\n", qreg_parse3);

    let int_parse1 = QuillParser::parse(Rule::Int, "01");
    println!("{:?}\n", int_parse1);
    // THIS SHOULD FAIL
    
}
