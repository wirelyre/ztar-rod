pub use super::super::datatype::DataType;

#[derive(Debug, Clone)]
pub struct Script(Vec<Declaration>);

#[derive(Debug, Clone)]
pub enum Declaration {
    Fun {
        name:      Identifier,
        arguments: Vec<(Identifier, DataType)>,
        block:     Vec<Statement>,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    Return,

    Label { name: String },
    Goto  { label_name: String },

    VarAssign {
        identifier: Identifier,
        expression: Expression,
    },

    VarDeclare {
        datatype:    DataType,
        identifiers: Vec<Identifier>,
        expression:  Option<Expression>,
    },

    MethodCall {
        method:    Method,
        arguments: Vec<Expression>,
        threading: MethodThreadType,
    },

    Wait { time: Expression, unit: TimeUnit },

    If {
        condition:   Expression,
        block_true:  Vec<Statement>,
        block_false: Vec<Statement>,
    },

    Switch {
        expression: Expression,
        cases:      Vec<(Case, Vec<Statement>)>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    LiteralInt(u32),
    LiteralFloat(f32),
    LiteralBool(bool),

    Identifier(Identifier),
    ArrayIndex(Identifier, u8),

    Operation {
        lhs: Box<Expression>,
        op:  Operator,
        rhs: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Method {
    Identifier(Identifier),
    Pointer(u32),
}

#[derive(Debug, Clone)]
pub enum MethodThreadType {
    No,                 // method()
    Yes,                // thread method()
    Assign(Identifier), // var = thread method()
}

#[derive(Debug, Clone)]
pub enum TimeUnit {
    Frames,
    Seconds,
}

#[derive(Debug, Clone)]
pub enum Case {
    Default,
    Test {
        operator: Operator,
        against:  Expression,
    },
}

#[derive(Debug, Clone)]
pub enum Operator {
    // Arithmetic
    Add, Sub, Mul, Div, Mod,

    // Logic
    Eq, Ne, Gt, Lt, Gte, Lte,
    BitAndZ, BitAndNz,
    And, Or, Not,
    CaseDefault,
}

#[derive(Debug, Clone)]
pub struct Identifier(pub String);
