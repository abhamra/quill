/// Define optimization passes + pass to check number of Maistow's vs All Assignments for ratio
/// calculation

use crate::ast::{ASTNode, RespectExpr, GateExpr, OutputExpr, NodeKind};

// This function calculates whether the correct ratio of
// (change back to doc comment when fixed)
pub fn respect_ratio(ast: &ASTNode) -> bool {
    let mut maistows: f64 = 0.0;
    let mut total: f64 = 0.0;
    for node in ast.children.as_ref().unwrap() {
        match node.node_kind {
            NodeKind::Assignment => {
                total += 1.0;
                if let Some(children) = &node.children {
                    // TODO: try to destructure to directly match with the nodeKind
                    match &children[0].node_kind {
                        NodeKind::RespectType(typ) => {
                            match typ {
                                RespectExpr::Maistow => maistows += 1.0,
                                _ => {},
                            }
                        },
                        unknown => panic!("{:?} should not be the first child of Assignment Nodes!", unknown),
                    }
                }
            },
            _ => {},
        }
    }
    0.5 <= (maistows / total) && (maistows / total) <= 0.9
}

