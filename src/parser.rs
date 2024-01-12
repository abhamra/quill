use pest::Parser;
use pest_derive::Parser;
use pest::error::Error;
use crate::ast::{ASTNode, RespectExpr, GateType, OutputExpr};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct QuillParser;

pub fn parse(source: &str) -> Result<Vec<Box<ASTNode>>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = QuillParser::parse(Rule::Program, source)?;
    
    for pair in pairs {
        // match pair.as_rule() {
        //     Rule::Stmt => {
        //         ast.push(Box::new(build_node(pair).unwrap()));
        //     },
        //     _ => {}
        // }
        ast.push(Box::new(build_node(pair).unwrap()));
        // println!("{:?}", pair);
    }
    // For future ref, I want to somehow calculate the respect ratio and store it somewhere
    // we can probably return a tuple?

    Ok(ast)
}

fn build_node(pair: pest::iterators::Pair<Rule>) -> Option<ASTNode> {
    match pair.as_rule() {
        // Rule::Stmt => build_node(pair.into_inner().next()?),
        Rule::AssignStmt => {
            let mut pair = pair.into_inner();
            let respect = match pair.next()?.as_str() {
                "Maistow" => RespectExpr::Maistow,
                "Canstow" => RespectExpr::Canstow,
                _ => panic!("Unrecognized respect expression!"),
            };
            let name = build_node(pair.next()?)?;
            let value = build_node(pair.next()?)?;

            Some(ASTNode::Assignment{ respect: respect, name: Box::new(name), value: Box::new(value) })
        },
        Rule::GateStmt => {
            let mut pair = pair.into_inner();
            let gate_type = pair.next()?;
            let target = build_node(pair.next()?)?;
            let mut controls = None;
            let mut params = None;
            loop {
                if let Some(next_rule) = pair.next() {
                    match next_rule.as_rule() {
                        Rule::ControlList => {
                            controls = Some(Box::new(build_node(next_rule)?));
                        },
                        Rule::ValList => {
                            params = Some(Box::new(build_node(next_rule)?));
                            break;
                        },
                        _ => panic!("Gate Application Statement does not support {} yet!", next_rule.as_str()),
                    }
                } else {
                    break;
                }
            } 
            build_gate_app(gate_type, target, controls, params)
        },
        Rule::MeasureStmt => {
            let mut pair = pair.into_inner();
            let measured = build_node(pair.next()?)?; // Either Name or QRegSlice
            let recipient = build_node(pair.next()?)?; // Either Name or CRegSlice
            Some(ASTNode::Measurement {
                measured: Box::new(measured),
                recipient: Box::new(recipient),
            })
        },
        Rule::ReturnStmt => {
            let mut pair = pair.into_inner();
            let shots = Box::new(build_node(pair.next()?)?); // Int Node
            let mut output_type = None;
            if let Some(next_rule) = pair.next() {
                output_type = Some(next_rule.as_str().to_string());
            }
            Some(ASTNode::Return{
                shots: shots,
                output_type: output_type,
            })
        },
        Rule::QReg => {
            let mut pair = pair.into_inner();
            let qubit = Box::new(build_node(pair.next()?)?);
            let length = pair.next()?.as_str().parse::<i32>()
                .expect("Your QReg length could not be parsed as an int!");
            Some(ASTNode::QReg{
                qubit: qubit,
                length: length,
            })
        },
        Rule::QRegTensor => {
            let mut pair = pair.into_inner();
            let mut qregs = vec![];
            loop {
                if let Some(next_qreg) = pair.next() {
                    qregs.push(build_node(next_qreg)?);
                } else {
                    break;
                }
            }
            Some(ASTNode::QRegTensor(qregs))
        },
        Rule::QRegSlice => {
            let mut pair = pair.into_inner();
            let name = Box::new(build_node(pair.next()?)?);
            let mut indices = vec![];
            let ind1 = pair.next()?.as_str().parse::<i32>()
                .expect("Index could not be parsed as an int!");
            indices.push(ind1);
            if let Some(next_rule) = pair.next() {
                indices.push(next_rule.as_str().parse::<i32>()
                .expect("Index could not be parsed as an int!"));
            }
            Some(ASTNode::QRegSlice{
                name: name,
                indices: indices,
            })
        },
        Rule::Qubit => {
            Some(ASTNode::Qubit(pair.as_str().to_string()))
        },   
        Rule::CReg => {
            let mut pair = pair.into_inner();
            let cbit = Box::new(build_node(pair.next()?)?);
            let length = pair.next()?.as_str().parse::<i32>()
                .expect("Your QReg length could not be parsed as an int!");
            Some(ASTNode::CReg {
                cbit: cbit,
                length: length,
            })
        },
        Rule::CRegTensor => {
            let mut pair = pair.into_inner();
            let mut cregs = vec![];
            loop {
                if let Some(next_creg) = pair.next() {
                    cregs.push(build_node(next_creg)?);
                } else {
                    break;
                }
            }
            Some(ASTNode::CRegTensor(cregs))
        },
        Rule::CRegSlice => {
            let mut pair = pair.into_inner();
            let name = Box::new(build_node(pair.next()?)?);
            let mut indices = vec![];
            let ind1 = pair.next()?.as_str().parse::<i32>()
                .expect("Index could not be parsed as an int!");
            indices.push(ind1);
            if let Some(next_rule) = pair.next() {
                indices.push(next_rule.as_str().parse::<i32>()
                .expect("Index could not be parsed as an int!"));
            }
            Some(ASTNode::CRegSlice{
                name: name,
                indices: indices,
            })
        },
        Rule::CBit => {
            let cbit = (pair.as_str().as_bytes()[1] as char).to_digit(10).unwrap();
            Some(ASTNode::CBit(cbit.try_into().unwrap()))
        },
        Rule::Index => {
            let index = pair.as_str().parse::<i32>().ok()?;
            Some(ASTNode::Index(index))
        },
        Rule::Int => {
            let int_str = pair.as_str();
            let (sign, val) = match &int_str[..1] {
                "-" => (-1, &int_str[1..]),
                _   => (1, int_str),
            };
            let final_val:i32 = val.parse().unwrap();
            Some(ASTNode::Int(sign * final_val))
        },
        Rule::Float => {
            let float_str = pair.as_str();
            let (sign, val) = match &float_str[..1] {
                "-" => (-1, &float_str[1..]),
                _   => (1, float_str),
            };
            let final_val:f64 = val.parse().unwrap();
            Some(ASTNode::Float((sign as f64) * final_val))
        },
        Rule::PI => {
            let mut pair = pair.into_inner();
            let mut pi_mult = std::f64::consts::PI;
            if let Some(next_rule) = pair.next() {
                pi_mult *= next_rule.as_str().parse::<f64>().ok()?;
                if let Some(second_rule) = pair.next() {
                    pi_mult /= second_rule.as_str().parse::<f64>().ok()?;
                }
            }
            Some(ASTNode::PI(pi_mult))
        },
        Rule::Name => {
            Some(ASTNode::Name(pair.as_str().to_string()))
        },
        Rule::ValList => {
            let mut val_list = vec![];
            for pair in pair.into_inner() {
                val_list.push(build_node(pair)?);
            }
            Some(ASTNode::ValList(val_list))
        },
        Rule::ControlList => {
            let mut control_list = vec![];
            for pair in pair.into_inner() {
                control_list.push(build_node(pair)?);
            }
            Some(ASTNode::ValList(control_list))
        },
        Rule::OutputType => {
            let out_type = match pair.into_inner().as_str() {
                "qir" => OutputExpr::QIR,
                "qiskit" => OutputExpr::Qiskit,
                "qasm" => OutputExpr::QASM,
                &_ => panic!("Unknown output type found!"),
            };
            Some(ASTNode::OutputType(out_type))
        },
        Rule::EOI => Some(ASTNode::EOI),
        unknown => {
            println!("currently undefined rule: {:?}", unknown);
            Some(ASTNode::Unimplemented)
        },
    }
}

// Helper functions for creating all of the different ASTNodes
fn build_gate_app(gate_rule: pest::iterators::Pair<Rule>, target: ASTNode, controls: Option<Box<ASTNode>>, params: Option<Box<ASTNode>>) -> Option<ASTNode> {
    let gate_type = match gate_rule.clone().as_rule() {
        Rule::Q1Gate => GateType::Q1Gate,
        Rule::Q1ParamGate => GateType::Q1ParamGate,
        Rule::Q2Gate => GateType::Q2Gate,
        Rule::Q2ParamGate => GateType::Q2ParamGate,
        Rule::QMultiGate => GateType::QMultiGate,
        unknown => panic!("Unknown gate type given: {:?}", unknown),
    };
    let gate = String::from(gate_rule.as_str());

    Some(ASTNode::GateApplication { 
        gate: gate,
        gate_type: gate_type,
        target: Box::new(target),
        controls: controls,
        params: params,
    })
}
