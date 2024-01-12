// For program, make the node have a "body" which is a vec of ASTNodes
//
// The CreateLang docs basically make it seem like the vec of nodes is there so that it can act as
// an overarching structure to hold all of the sequentially appearing "parent" nodes of each
// expression, but this is EXACTLY what we want our Program node to do. Because of this, I think
// that after our parsing, it may be more useful to actually just create the program manually and
// instantiate it with the mut Vec<Node> ast that we create earlier.
//
// IDEAS:
// - The node should be an enum and could encode the parent, important values, and children of the
// node
// Program,
// Assignment,
// GateApp,
// Measurement,
// Return,
// Name
// QRegSlice
// Value
// RespectExpr

use self::ASTNode::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum RespectExpr {
    Canstow,
    Maistow,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum OutputExpr {
    Qiskit, 
    QASM, 
    QIR
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum GateType {
    Q1Gate,
    Q1ParamGate,
    Q2Gate,
    Q2ParamGate,
    ToffoliGate,
    QMultiGate,
}

#[derive(PartialEq, Debug, Clone)]
pub enum ASTNode { 
    Assignment {
        respect: RespectExpr,
        // r#type: String, // Unnecessary, the value node will encode type info
        name: Box<ASTNode>, // Name
        value: Box<ASTNode> // One of the value nodes
    }, 
    GateApplication {
        gate: String,
        gate_type: GateType,
        target: Box<ASTNode>, // Name
        controls: Option<Box<ASTNode>>, // ControlList
        params: Option<Box<ASTNode>>, // ValList
    },
    Measurement {
        measured: Box<ASTNode>, // Name
        recipient: Box<ASTNode>, // Name
    },
    Return {
        shots: Box<ASTNode>, // Integer Value
        output_type: Option<String>,
    },
    Name(String),
    QRegSlice {
        name: Box<ASTNode>,
        indices: Vec<i32>, // parse from indices
    },
    CRegSlice {
        name: Box<ASTNode>,
        indices: Vec<i32>,
    },
    ValList(Vec<ASTNode>), // Simple list of Values
    ControlList(Vec<ASTNode>), // List of Names (Controlled qubits)
    QRegTensor(Vec<ASTNode>), // Vec of QRegs
    QReg {
        qubit: Box<ASTNode>, // Qubit
        length: i32, // Index
    },
    Qubit(String), // fixing
    CRegTensor(Vec<ASTNode>), // Vec of CRegs
    CReg {
        cbit: Box<ASTNode>, // CBit
        length: i32,
    },
    CBit(i32),
    Float(f64),
    Int(i32),
    Index(i32),
    PI(f64),
    OutputType(OutputExpr),
    EOI,
    Unimplemented, // THIS IS TEMPORARY!!!
}
