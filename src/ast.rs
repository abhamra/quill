#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum RespectExpr {
    Canstow,
    Maistow,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum GateExpr {
    Q1Gate,
    Q1ParamGate,
    Q2Gate,
    Q2ParamGate,
    ToffoliGate,
    QMultiGate,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ValueExpr {
    QReg,
    Qubit,
    CBit,
    CReg,
}

#[derive(PartialEq, Debug, Clone)]
pub enum NodeKind {
    // Question: Do we want to add data / fields to the NodeKind enum variants?
    Program,
    Assignment,
    GateApplication,
    Measurement,
    Return,
    Name(String),
    Indices, // Indices(Vec<i32>) was alternative, but for now we hold the indices as children
    QRegSlice,
    CRegSlice,
    ValList, // ValList(Vec<ASTNode>) was alternative, for now we hold values as children
    ControlList,
    QRegTensor, // Children will be QRegs
    QReg,       // Children will be qubit, Index
    Qubit(String),
    CRegTensor,
    CReg, // Children will be CBit, Index
    CBit(i32),
    Float(f64),
    Int(i32),
    Index(i32),
    PI(f64),
    ValueType(ValueExpr),
    GateType(GateExpr),
    RespectType(RespectExpr),
    COMMENT,
    EOI,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ASTNode {
    pub children: Option<Vec<ASTNode>>,
    pub node_kind: NodeKind,
}

// NOTE: The Rust Compiler uses the below struct to define an expression
// #[derive(Clone, Encodable, Decodable, Debug)]
// pub struct Expr {
//     pub id: NodeId,
//     pub kind: ExprKind,
//     pub span: Span,
//     pub attrs: AttrVec,
//     pub tokens: Option<LazyAttrTokenStream>,
// }
// Followed by an impl block for Expr
// The ExprKind enum is also important, and is implemented with pointers to
// expressions like below:
// #[derive(Clone, Encodable, Decodable, Debug)]
// pub enum ExprKind {
//      Binary(BinOp, P<Expr>, P<Expr>)
//      // NOTE: here, BinOp is an enum, and P is from rustc_ast::ptr::P,
//  } (this is just an example, it continues for a while)
//
//  The implementation of rustc_ast::ptr::P is below, just for clarity (it is
//  quite literally a wrapper around Box)
//  pub struct P<T: ?Sized> {
//     ptr: Box<T>,
// }

// TODO: Create a "new" function, and functionalize some code in parser (the ones that parse lists
// of objects are suitable targets for this)
// Also, (potentially) create functions to modify children and node_kind, so that the fields don't
// have to be public (for safety, use accessors and mutator methods)

impl ASTNode {
    pub fn new(children: Option<Vec<ASTNode>>, node_kind: NodeKind) -> ASTNode {
        ASTNode {
            children: children,
            node_kind: node_kind,
        }
    }

    pub fn print_nodes(node: &ASTNode, depth: usize) {
        println!("{}{:?}", String::from("    ").repeat(depth), node.node_kind);
        for child in &node.children {
            for c in child {
                ASTNode::print_nodes(&c, depth + 1)
            }
        }
    }
}
