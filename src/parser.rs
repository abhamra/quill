use pest::Parser;
use pest_derive::Parser;
use pest::error::Error;
use crate::ast::{ASTNode, RespectExpr, GateType};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct QuillParser;

pub fn parse(source: &str) -> Result<Vec<Box<ASTNode>>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = QuillParser::parse(Rule::Program, source)?;
    
    for pair in pairs {
        match pair.as_rule() {
            Rule::Stmt => {
                ast.push(Box::new(build_node(pair).unwrap()));
            },
            _ => {}
        }
    }
    // For future ref, I want to somehow calculate the respect ratio and store it somewhere
    // we can probably return a tuple?

    Ok(ast)
}

// Helper functions for creating all of the different ASTNodes
fn build_gate_app(gate_rule: pest::iterators::Pair<Rule>, target: ASTNode, controls: Option<Vec<ASTNode>>, params: Option<Vec<ASTNode>>) -> Option<ASTNode> {
    let gate_type = match gate_rule.clone().as_rule() {
        Rule::Q1Gate => GateType::Q1Gate,
        Rule::Q1ParamGate => GateType::Q1ParamGate,
        Rule::Q2Gate => GateType::Q2Gate,
        Rule::Q2ParamGate => GateType::Q2ParamGate,
        Rule::QMultiGate => GateType::QMultiGate,
    }
    let gate = String::from(gate_rule.as_str());

    Some(ASTNode::GateApplication { 
        gate: gate,
        gate_type: gate_type,
        target: target,
        controls: controls,
        params: params,
    })
}

fn build_node(pair: pest::iterators::Pair<Rule>) -> Option<ASTNode> {
    match pair.as_rule() {
        Rule::AssignStmt => {
            let mut pair = pair.into_inner();
            let respect = match pair.next()?.as_str() {
                "Maistow" => RespectExpr::Maistow,
                "Canstow" => RespectExpr::Canstow,
                _ => panic!("Unrecognized respect expression!");
            };
            let name = build_node(pair.next()?)?;
            let value = build_node(pair.next()?)?;

            Some(ASTNode::Assignment{ respect: String::from(respect), name: Box::new(name), value: Box::new(value) })
        },
        Rule::GateStmt => {
            let mut pair = pair.into_inner();
            let gate_type = pair.next()?;
            let target = build_node(pair.next()?)?;
            let mut controls = None;
            let mut params = None;
            loop {
                if let Some(next_rule) = pair.next() {
                    match next_rule {
                        Rule::ControlList => {
                            controls = build_node(next_rule);
                        },
                        Rule::ValList => {
                            params = build_node(next_rule);
                            break;
                        },
                        _ => panic!("Gate Application Statement does not support {} yet!", next_rule.as_str());
                    }
                } else {
                    break;
                }
            } 
            build_gate_app(gate_type, target, controls, params)
        },
        Rule::MeasureStmt => {
            
        },
        Rule::ReturnStmt => {
            
        },
        Rule::QReg => {

        },
        Rule::QRegTensor => {

        },
        Rule::QRegSlice => {

        },
        Rule::Qubit {

        },   
        Rule::CReg => {

        },
        Rule::CRegTensor => {
    
        },
        Rule::CRegSlice => {

        },
        Rule::CBit => {

        },
        Rule::Index => {

        },
        Rule::Int => {

        },
        Rule::Float => {

        },
        Rule::Name => {
            let name =  pair.into_inner().next()?;
            Some(ASTNode::Name(name.as_str().to_string()))
        },
        Rule::ValList => {
            let mut val_list = vec![];
            for pair in pair.into_inner() {
                val_list.push(build_node(pair.next()?)?);
            }
            Some(ASTNode::ValList(val_list))
        },
        Rule::ControlList => {
            let mut control_list = vec![];
            for pair in pair.into_inner() {
                control_list.push(build_node(pair.next()?)?);
            }
            Some(ASTNode::ValList(control_list))
        }
    }
}
