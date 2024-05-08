//! Basic type checker for verifying validity of Quill programs

use crate::ast::{ASTNode, NodeKind, ValueExpr};
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

    for node in ast.children.as_ref().unwrap() {
        match &node.node_kind {
            NodeKind::Assignment => {
                let children = node.children.as_ref().unwrap();
                assert_eq!(children.len(), 4, "Your assignment node somehow didn't have the requisite number of elements!\nShame on thee!");
                let val_type = &children[1];
                let name = match &children[2].node_kind {
                    NodeKind::Name(nam) => nam,
                    _ => unreachable!(),
                };
                let value = &children[3];

                let val_expr = match &val_type.node_kind {
                    NodeKind::ValueType(typ) => assignment_helper(typ, &value),
                    _ => panic!("The ValueType node should have AST NodeKind ValueType!"),
                };
                // Use the return value of insert to check and see if there was a previous entry
                // with the same name, and then verify types!
                let old_val = ctx.insert((*name.clone()).to_string(), val_expr.clone());
                if let Some(prev) = old_val {
                    if prev != val_expr {
                        panic!(
                            "{:?} was originally of type {:?}, but now given type {:?}!",
                            *name,
                            prev,
                            val_expr.clone()
                        );
                    }
                }
            }
            NodeKind::GateApplication => {
                let children = node.children.as_ref().unwrap();
                // [gate, gate_type_node, target, controls, params] (Always controls first)
                // controls and params are optional

                // Check name of target, verify that it's qubit or single qreg slice
                // OR, if is multi qreg slice, then the gate is a single qubit gate of some form

                match &children[2].node_kind {
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
                        let qreg_children = &children[2].children.as_ref().unwrap();
                        println!("{:?}", qreg_children); // [name, indices_node]
                        match &qreg_children[0].node_kind {
                            NodeKind::Name(nam) => {
                                // QReg Case, verify name is a QReg
                                if let Some(val) = ctx.get(nam) {
                                    match *val {
                                        ValueExpr::QReg => {
                                            // TODO: Check index validity
                                        }
                                        _ => panic!("Qubit expected, {:?} given!", val.clone()),
                                    }
                                } else {
                                    panic!(
                                        "Unknown variable {:?} given, not a qubit!",
                                        nam.clone()
                                    );
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
                }

                // actually do the type checking for the
                // [gate, gate_type_node, target, controls, params] (Always controls first)
                match children.len() {
                    3 => {}
                    4 => {}
                    5 => {}
                    _ => unreachable!(),
                }

                // Note: Make sure to use a match against the length of the children vec,
                // to see if you have a controls list or params list to care about
                // Note 2: If there are 3 elts, always a control list (not param list)
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
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
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
                            _ => unreachable!(),
                        }
                    }
                    _ => unreachable!(),
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
