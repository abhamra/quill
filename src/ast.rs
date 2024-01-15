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
        output_type: Option<Box<ASTNode>>, // OutputType
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
}
