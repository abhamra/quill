//! Basic type checker for verifying validity of Quill programs

use crate::ast::{ASTNode, RespectExpr, GateExpr, OutputExpr, ValueExpr, NodeKind};
use std::collections::HashMap;

// NOTE TO SELF: MAKE CUSTOM ERRORS FOR TYPE CHECKER

/// This function is for type checking the AST, making sure that
/// all of the statements are valid, typing wise
pub fn type_check(ast: &ASTNode) {
    // Stores entries of <Variable Name, Type>
    let mut ctx: HashMap<String, ValueExpr> = HashMap::new();

    // NOTE: CREATE CUSTOM ERROR TYPES
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
                        panic!("{:?} was originally of type {:?}, but now given type {:?}!", *name, prev, val_expr.clone());
                    }
                }
            },
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
                                ValueExpr::Qubit => {},
                                _ => panic!("Qubit expected, {:?} given!", val.clone()),
                            }
                        } else {
                            panic!("Unknown variable {:?} given, not a qubit!", nam.clone());
                        }
                    },
                    NodeKind::QRegSlice => {
                        let qreg_children = &children[2].children.as_ref().unwrap();
                        println!("{:?}", qreg_children);
                    },
                    _ => unreachable!(),
                }


                match children.len() {
                    3 => {},
                    4 => {},
                    5 => {},
                    _ => unreachable!(),   
                }

                // Note: Make sure to use a match against the length of the children vec, 
                // to see if you have a controls list or params list to care about
                // Note 2: If there are 3 elts, always a control list (not param list)
            },
            NodeKind::Measurement => {},
            NodeKind::Return => {},
            NodeKind::EOI => {},
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
