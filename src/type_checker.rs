//! Basic type checker for verifying validity of Quill programs

use crate::ast::{ASTNode, GateExpr, NodeKind, ValueExpr};
use std::collections::HashMap;

/// This function is for type checking the AST, making sure that
/// all of the statements are valid, typing wise
pub fn type_check(ast: &ASTNode) {
    // Stores entries of <Variable Name, Type>
    let mut ctx: HashMap<String, ValueExpr> = HashMap::new();

    // Check to see if Return is the second last child
    let children = ast.children.as_ref().unwrap();
    for i in 0..(children.len() - 1) {
        // children.len() - 1 to ignore EOI
        if children[i].node_kind == NodeKind::Return && i != (children.len() - 2) {
            panic!(
                "Return expected on line {}, found at line {} instead!",
                children.len() + 1,
                i + 2
            );
        }
    }

    // What I need to check for:
    // - If a variable of a given type was instantiated with that type correctly
    // - If there is an assignment of an existing variable name, make sure the old type is adhered
    // to
    // - If gate expressions have gates being applied correctly
    // - If gate expressions apply a gate to a given qubit, make sure it exists
    // - If gate expressions apply a gate to a qreg, make sure the slice exists
    //   (This can include a single index -> if out of bounds, bad, or it can
    //   also include having a range and the range not being valid)
    // - If gate expressions apply to a given qubit, make sure there are no duplicate names
    // - If gate expressions apply to a given qreg, make sure the slices do not overlap at all
    // - For multicontrol gates, make sure the slices are limited to one single instance of the
    //   qreg and not multiple qubits across the qreg
    // - For the measurement expressions, check that the number of qubits measured and number of
    // cbits measured matches up
    // - If a return statement is caught but is not the last line of the program, throw an
    // error
    // - Make sure that for all nodes, they have the correct amount of children (if they have any
    // at all, also check that) for example, EOI shouldnt have children
    // let len = ast.children.as_ref().unwrap().len();
    let mut line_no = 0;
    for node in ast.children.as_ref().unwrap() {
        match &node.node_kind {
            NodeKind::Assignment => {
                line_no += 1;
                let children = node.children.as_ref().unwrap();
                assert_eq!(children.len(), 4, "{}: Your assignment node somehow didn't have the requisite number of elements!\nShame on thee!", line_no);
                let val_type = &children[1];
                let name = match &children[2].node_kind {
                    NodeKind::Name(nam) => nam,
                    _ => unreachable!(),
                };
                let value = &children[3];

                let val_expr = match &val_type.node_kind {
                    NodeKind::ValueType(typ) => assignment_helper(typ, &value),
                    _ => panic!(
                        "{}: The ValueType node should have AST NodeKind ValueType!",
                        line_no
                    ),
                };
                // Use the return value of insert to check and see if there was a previous entry
                // with the same name, and then verify types!
                let old_val = ctx.insert((*name.clone()).to_string(), val_expr.clone());
                if let Some(prev) = old_val {
                    if prev != val_expr {
                        panic!(
                            "{}: {:?} was originally of type {:?}, but now given type {:?}!",
                            line_no,
                            *name,
                            prev,
                            val_expr.clone()
                        );
                    }
                }
            }
            NodeKind::GateApplication => {
                line_no += 1;
                let children = node.children.as_ref().unwrap();
                // [gate, gate_type_node, target, controls, params] (Always controls first)
                // controls and params are optional

                // Check name of target, verify that it's qubit or single qreg slice
                // OR, if is multi qreg slice, then the gate is a single qubit gate of some form
                verify_target(&children[2], &ctx, line_no);
                // match &children[2].node_kind {
                //     NodeKind::Name(nam) => {
                //         // Qubit Case, verify name is a qubit
                //         if let Some(val) = ctx.get(nam) {
                //             match *val {
                //                 ValueExpr::Qubit => {}
                //                 _ => panic!("Qubit expected, {:?} given!", val.clone()),
                //             }
                //         } else {
                //             panic!("Unknown variable {:?} given, not a qubit!", nam.clone());
                //         }
                //     }
                //     NodeKind::QRegSlice => {
                //         let qreg_children = &children[2].children.as_ref().unwrap();
                //         match &qreg_children[0].node_kind {
                //             /*TODO: Should probably be an if let*/
                //             NodeKind::Name(nam) => {
                //                 // QReg Case, verify name is a QReg
                //                 if let Some(val) = ctx.get(nam) {
                //                     match *val {
                //                         ValueExpr::QReg => {
                //                             // TODO: Check index validity
                //                         }
                //                         _ => panic!("QReg expected, {:?} given!", val.clone()),
                //                     }
                //                 } else {
                //                     panic!("Unknown variable {:?} given, not a qreg!", nam.clone());
                //                 }
                //             }
                //             _ => unreachable!(),
                //         }
                //     }
                //     _ => unreachable!(),
                // }

                // actually do the type checking for the
                // [gate, gate_type_node, target, controls, params] (Always controls first) (match against gate_type_node for what to expect)
                // NOTE: this if let is kind of redundant, but "easy" for now
                // note that gates, as talked about below, will always be vacuously
                // correct. Below, we destructure the vector manually because it
                // isn't cool like tuples.
                let gate_node_kind = &children[1].node_kind;
                // let controls = &children[3]; // Option
                // let params = &children[4]; // Option
                if let NodeKind::GateType(gate_expr) = gate_node_kind {
                    match gate_expr {
                        GateExpr::Q1Gate => {
                            /*children.len() = 3, no cont, params
                             * we have already checked for target's validity
                             * and because of pest parsing, the gate name will
                             * be a correct subset of the gate_type_node's
                             * category (e.g. 'h' will necessarily be of
                             * type Q1Gate. Hence, we do nothing here!*/
                        }
                        GateExpr::Q1ParamGate => {
                            /*Requires params list, so check for params.len() == 1*/
                            if let Some(pars) = &children[3].children {
                                assert!(
                                    pars.len() == 1,
                                    "{}", format!("{}: More than one parameter for single qubit parameterized gate!", line_no)
                                );
                                // DONE: Continue checks by making sure the parameter is defined
                                // correctly (Has to be PI, Float, Int for params, make this check
                                // a function?)
                                match &pars[0].node_kind {
                                    NodeKind::PI(_) | NodeKind::Float(_) | NodeKind::Int(_) => {},
                                    other => panic!("{}: Parameters should be of type PI, Float, or Int, found {:?} instead!", line_no, other),
                                }
                            } else {
                                panic!("{}: No parameters for Q1 Param Gate!", line_no);
                            }
                        }
                        GateExpr::Q2Gate => {
                            /*Q2 gates are cx, cz for now, so they require controls, not params*/
                            if let Some(controls) = &children[3].children {
                                assert!(
                                    controls.len() == 1,
                                    "{}", format!("{}: More than one controlled qubit for a double qubit control gate!", line_no)
                                );
                                // TODO: Check that control is a defined qubit and not a duplicate
                                // (make function)
                            } else {
                                panic!("{}: Q2 gates require controls list, but no list of controlled qubits was found!", line_no);
                            }
                        }
                        GateExpr::Q2ParamGate => {
                            /*Requires params list, so check for params.len() == 1*/
                            if let Some(pars) = &children[3].children {
                                assert!(
                                    pars.len() == 1,
                                    "{}", format!("{}: More than one parameter for single qubit parameterized gate!", line_no)
                                );
                                // DONE: Continue checks by making sure the parameter is defined
                                // correctly (Has to be PI, Float, Int for params, make this check
                                // a function?)
                                match &pars[0].node_kind {
                                    NodeKind::PI(_) | NodeKind::Float(_) | NodeKind::Int(_) => {},
                                    other => panic!("{}: Parameters should be of type PI, Float, or Int, found {:?} instead!", line_no, other),
                                }
                            } else {
                                panic!("{}: No parameters for Q2 Param Gate!", line_no);
                            }
                        }
                        GateExpr::QMultiGate => {
                            println!("Found QMultiGate!");
                        }
                        _ => {}
                    }
                }
            }
            NodeKind::Measurement => {
                // [measured (Name/QRegSlice), recipient (Name/CRegSlice)]
                let children = node.children.as_ref().unwrap();

                // Measured Qubit / QRegSlice
                match &children[0].node_kind {
                    NodeKind::Name(nam) => {
                        // Qubit Case, verify name is a qubit
                        if let Some(val) = ctx.get(nam) {
                            match *val {
                                ValueExpr::Qubit => {}
                                _ => panic!("Qubit expected, {:?} given!", val.clone()),
                            }
                        } else {
                            panic!("Unknown variable {:?} given, not a qubit!", nam.clone());
                        }
                    }
                    NodeKind::QRegSlice => {
                        let qreg_children = &children[0].children.as_ref().unwrap();
                        match &qreg_children[0].node_kind {
                            NodeKind::Name(nam) => {
                                // QReg Case, verify name is a QReg
                                if let Some(val) = ctx.get(nam) {
                                    match *val {
                                        ValueExpr::QReg => {
                                            // TODO: Check index validity
                                        }
                                        _ => panic!("CReg expected, {:?} given!", val.clone()),
                                    }
                                } else {
                                    panic!(
                                        "Unknown variable {:?} given, not a qubit!",
                                        nam.clone()
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                } // end of match

                // Recipient CBit / CRegSlice
                match &children[1].node_kind {
                    NodeKind::Name(nam) => {
                        // CBit case, verify name is a cbit
                        if let Some(val) = ctx.get(nam) {
                            match *val {
                                ValueExpr::CBit => {}
                                _ => panic!("CBit expected, {:?} given!", val.clone()),
                            }
                        } else {
                            panic!("Unknown variable {:?} given, not a CBit!", nam.clone());
                        }
                    }
                    NodeKind::CRegSlice => {
                        let creg_children = &children[0].children.as_ref().unwrap();
                        match &creg_children[0].node_kind {
                            NodeKind::Name(nam) => {
                                // QReg Case, verify name is a QReg
                                if let Some(val) = ctx.get(nam) {
                                    match *val {
                                        ValueExpr::CReg => {
                                            // TODO: Check index validity
                                        }
                                        _ => panic!("CReg expected, {:?} given!", val.clone()),
                                    }
                                } else {
                                    panic!(
                                        "Unknown variable {:?} given, not a qubit!",
                                        nam.clone()
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            NodeKind::Return => {
                // [shots]
                // Verify if integer is non-negative
                let children = node.children.as_ref().unwrap();
                match &children[0].node_kind {
                    NodeKind::Int(val) => {
                        if *val < 1 {
                            panic!(
                                "Non-negative number of shots required, {} shots received instead!",
                                val
                            );
                        }
                    }
                    _ => panic!("Node is not of type int, unexpected in return statement!"),
                }
            }
            NodeKind::EOI => {} // Intentionally do nothing here, nothing to handle
            unknown => panic!("{:?} is not a valid top-level node!", unknown),
        }
    }
}

/// A helper function for the assignment portion of the type checker
/// which allows us to compare the type we've annotated and
/// the actual type of the variable!
fn assignment_helper(typ: &ValueExpr, value: &ASTNode) -> ValueExpr {
    let value_typ = match &value.node_kind {
        NodeKind::QReg => ValueExpr::QReg,
        NodeKind::CReg => ValueExpr::CReg,
        NodeKind::Qubit(_) => ValueExpr::Qubit,
        NodeKind::CBit(_) => ValueExpr::CBit,
        unknown => panic!("{:?} is not supported in assignment expressions!", unknown),
    };

    assert_eq!(*typ, value_typ);
    value_typ
}

// fn control_validity()
fn verify_target(target: &ASTNode, ctx: &HashMap<String, ValueExpr>, line_no: i32) {
    match &target.node_kind {
        NodeKind::Name(nam) => {
            // Qubit Case, verify name is a qubit
            if let Some(val) = ctx.get(nam) {
                match *val {
                    ValueExpr::Qubit => {}
                    _ => panic!("{}: Qubit expected, {:?} given!", line_no, val.clone()),
                }
            } else {
                panic!(
                    "{}: Unknown variable {:?} given, not a qubit!",
                    line_no,
                    nam.clone()
                );
            }
        }
        NodeKind::QRegSlice => {
            let qreg_children = target.children.as_ref().unwrap();
            match &qreg_children[0].node_kind {
                /*TODO: Should probably be an if let*/
                NodeKind::Name(nam) => {
                    // QReg Case, verify name is a QReg
                    if let Some(val) = ctx.get(nam) {
                        match *val {
                            ValueExpr::QReg => {
                                // TODO: Check index validity
                            }
                            _ => panic!("{}: QReg expected, {:?} given!", line_no, val.clone()),
                        }
                    } else {
                        panic!(
                            "{}: Unknown variable {:?} given, not a qreg!",
                            line_no,
                            nam.clone()
                        );
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}
