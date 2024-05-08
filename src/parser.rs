use crate::ast::{ASTNode, GateExpr, NodeKind, RespectExpr, ValueExpr};
use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;
use std::result::Result;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct QuillParser;

pub fn parse(source: &str) -> Result<ASTNode, Error<Rule>> {
    let pairs = QuillParser::parse(Rule::Program, source)?;

    let ast = pairs
        .into_iter()
        .map(|pair| build_node(pair).unwrap())
        .collect();

    let root = ASTNode {
        children: Some(ast),
        node_kind: NodeKind::Program,
    };

    Ok(root)
}

fn build_node(pair: pest::iterators::Pair<Rule>) -> Option<ASTNode> {
    match pair.as_rule() {
        Rule::AssignStmt => {
            let mut pair = pair.into_inner();
            let respect = match pair.next()?.as_str() {
                "Maistow" => RespectExpr::Maistow,
                "Canstow" => RespectExpr::Canstow,
                _ => panic!("Unrecognized respect expression!"),
            };
            let val_type = match pair.next()?.as_str() {
                "qreg" => ValueExpr::QReg,
                "qubit" => ValueExpr::Qubit,
                "creg" => ValueExpr::CReg,
                "cbit" => ValueExpr::CBit,
                unknown => panic!("{:?} is not a valid type to assign!", unknown),
            };
            let name = build_node(pair.next()?)?;
            let value = build_node(pair.next()?)?;

            let respect_node = ASTNode {
                children: None,
                node_kind: NodeKind::RespectType(respect),
            };
            let val_type_node = ASTNode {
                children: None,
                node_kind: NodeKind::ValueType(val_type),
            };

            Some(ASTNode {
                children: Some(vec![respect_node, val_type_node, name, value]),
                node_kind: NodeKind::Assignment,
            })
        }
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
                            controls = Some(build_node(next_rule)?);
                        }
                        Rule::ValList => {
                            params = Some(build_node(next_rule)?);
                            break;
                        }
                        _ => panic!(
                            "Gate Application Statement does not support {} yet!",
                            next_rule.as_str()
                        ),
                    }
                } else {
                    break;
                }
            }
            build_gate_app(gate_type, target, controls, params)
        }
        Rule::MeasureStmt => {
            let mut pair = pair.into_inner();
            let measured = build_node(pair.next()?)?; // Either Name or QRegSlice
            let recipient = build_node(pair.next()?)?; // Either Name or CRegSlice
            Some(ASTNode {
                children: Some(vec![measured, recipient]),
                node_kind: NodeKind::Measurement,
            })
        }
        Rule::ReturnStmt => {
            let mut pair = pair.into_inner();
            let shots = build_node(pair.next()?)?; // Int Node
            Some(ASTNode {
                children: Some(vec![shots]),
                node_kind: NodeKind::Return,
            })
        }
        Rule::QReg => {
            let mut pair = pair.into_inner();
            let qubit = build_node(pair.next()?)?;
            let length = build_node(pair.next()?)?; // Should parse to Index node correctly

            Some(ASTNode {
                children: Some(vec![qubit, length]),
                node_kind: NodeKind::QReg,
            })
        }
        Rule::QRegTensor => {
            let qregs: Vec<ASTNode> = pair
                .into_inner()
                .map(|pair| build_node(pair).unwrap())
                .collect();
            Some(ASTNode {
                children: Some(qregs),
                node_kind: NodeKind::QRegTensor,
            })
        }
        Rule::QRegSlice => {
            let mut pair = pair.into_inner();
            let name = build_node(pair.next()?)?;
            let mut indices = vec![];
            let ind1 = build_node(pair.next()?)?; // Parse to Index
            indices.push(ind1);
            if let Some(next_rule) = pair.next() {
                indices.push(build_node(next_rule)?);
            }
            let indices_node = ASTNode {
                children: Some(indices),
                node_kind: NodeKind::Indices,
            };
            Some(ASTNode {
                children: Some(vec![name, indices_node]),
                node_kind: NodeKind::QRegSlice,
            })
        }
        Rule::Qubit => Some(ASTNode {
            children: None,
            node_kind: NodeKind::Qubit(pair.as_str().to_string()),
        }),
        Rule::CReg => {
            let mut pair = pair.into_inner();
            let cbit = build_node(pair.next()?)?;
            let length = build_node(pair.next()?)?;
            Some(ASTNode {
                children: Some(vec![cbit, length]),
                node_kind: NodeKind::CReg,
            })
        }
        Rule::CRegTensor => {
            let cregs: Vec<ASTNode> = pair
                .into_inner()
                .map(|pair| build_node(pair).unwrap())
                .collect();
            Some(ASTNode {
                children: Some(cregs),
                node_kind: NodeKind::CReg,
            })
        }
        Rule::CRegSlice => {
            let mut pair = pair.into_inner();
            let name = build_node(pair.next()?)?;
            let mut indices = vec![];
            let ind1 = build_node(pair.next()?)?;
            indices.push(ind1);
            if let Some(next_rule) = pair.next() {
                indices.push(build_node(next_rule)?);
            }
            let indices_node = ASTNode {
                children: Some(indices),
                node_kind: NodeKind::Indices,
            };
            Some(ASTNode {
                children: Some(vec![name, indices_node]),
                node_kind: NodeKind::CRegSlice,
            })
        }
        Rule::CBit => {
            let cbit = (pair.as_str().as_bytes()[1] as char).to_digit(10).unwrap();
            Some(ASTNode {
                children: None,
                node_kind: NodeKind::CBit(cbit.try_into().unwrap()),
            })
        }
        Rule::Index => {
            let index = pair.as_str().parse::<i32>().ok()?;
            Some(ASTNode {
                children: None,
                node_kind: NodeKind::Index(index),
            })
        }
        Rule::Int => {
            let int_str = pair.as_str();
            let (sign, val) = match &int_str[..1] {
                "-" => (-1, &int_str[1..]),
                _ => (1, int_str),
            };
            let final_val: i32 = val.parse().unwrap();
            Some(ASTNode {
                children: None,
                node_kind: NodeKind::Int(sign * final_val),
            })
        }
        Rule::Float => {
            let float_str = pair.as_str();
            let (sign, val) = match &float_str[..1] {
                "-" => (-1, &float_str[1..]),
                _ => (1, float_str),
            };
            let final_val: f64 = val.parse().unwrap();
            Some(ASTNode {
                children: None,
                node_kind: NodeKind::Float((sign as f64) * final_val),
            })
        }
        Rule::PI => {
            let mut pair = pair.into_inner();
            let mut pi_mult = std::f64::consts::PI;
            if let Some(next_rule) = pair.next() {
                pi_mult *= next_rule.as_str().parse::<f64>().ok()?;
                if let Some(second_rule) = pair.next() {
                    pi_mult /= second_rule.as_str().parse::<f64>().ok()?;
                }
            }
            Some(ASTNode {
                children: None,
                node_kind: NodeKind::PI(pi_mult),
            })
        }
        Rule::Name => Some(ASTNode {
            children: None,
            node_kind: NodeKind::Name(pair.as_str().to_string()),
        }),
        Rule::ValList => {
            let val_list = pair
                .into_inner()
                .map(|pair| build_node(pair).unwrap())
                .collect();
            Some(ASTNode {
                children: Some(val_list),
                node_kind: NodeKind::ValList,
            })
        }
        Rule::ControlList => {
            let control_list = pair
                .into_inner()
                .map(|pair| build_node(pair).unwrap())
                .collect();
            Some(ASTNode {
                children: Some(control_list),
                node_kind: NodeKind::ControlList,
            })
        }
        Rule::EOI => Some(ASTNode {
            children: None,
            node_kind: NodeKind::EOI,
        }),
        _ => unimplemented!(),
    }
}

// Helper functions for creating all of the different ASTNodes
fn build_gate_app(
    gate_rule: pest::iterators::Pair<Rule>,
    target: ASTNode,
    controls: Option<ASTNode>,
    params: Option<ASTNode>,
) -> Option<ASTNode> {
    let gate_type = match gate_rule.clone().as_rule() {
        Rule::Q1Gate => GateExpr::Q1Gate,
        Rule::Q1ParamGate => GateExpr::Q1ParamGate,
        Rule::Q2Gate => GateExpr::Q2Gate,
        Rule::Q2ParamGate => GateExpr::Q2ParamGate,
        Rule::QMultiGate => GateExpr::QMultiGate,
        unknown => panic!("Unknown gate type given: {:?}", unknown),
    };

    let gate_type_node = ASTNode {
        children: None,
        node_kind: NodeKind::GateType(gate_type),
    };

    let gate = ASTNode {
        children: None,
        node_kind: NodeKind::Name(gate_rule.as_str().to_string()),
    };

    let mut children = vec![gate, gate_type_node, target];
    if let Some(cont) = controls {
        children.push(cont);
    }
    if let Some(pars) = params {
        children.push(pars);
    }

    Some(ASTNode {
        children: Some(children),
        node_kind: NodeKind::GateApplication,
    })
}
