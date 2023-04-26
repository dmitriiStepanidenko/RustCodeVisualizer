/// net_type ::=
/// supply0 | supply1 | tri | triand | trior | tri0 | tri1 | wire | wand | wor
/// Right now support only wire
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NetType {
    // Supply0,
    // Supply1,
    // Tri,
    // Triand,
    // Trior,
    // Tri0,
    // Tri1,
    Wire,
    // Wand,
    // Wor,
}

/// # Statement
///
/// Позволяет определять операции
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
    Wire(Wire),
    Register(Register),
    Assign(Assign),
    Always(Always),
    LocalParam(LocalParam),
    If(If),
    Case(Case),
    Assignment(Assignment),
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// TODO: Заменить строку на ссылку
/// Разница между Assignment в том, что это чисто под операции, начинающиеся на assign
pub struct Assign {
    pub left: String,
    pub right: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Always {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// TODO: Заменить строку на ссылку
pub enum Expression {
    Identifier(String),
    Unary(UnaryOp, Box<Expression>),
    Binary(Box<Expression>, BinaryOp, Box<Expression>),
    Number(Number),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
}

/// Бинарные операции:
/// - And
/// - Or
/// - Eq
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    And,
    Or,
    Eq,
}

/// Порты. Могут быть:
/// - Input
/// - Output
/// - Inout
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Port {
    Input(Input),
    Output(Output),
    Inout(Inout),
}

trait PortTrait {
    fn what_type(&self) -> String;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct If {
    pub condition: Expression,
    pub then_statements: Vec<Statement>,
    pub else_statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Case {
    pub expression: Expression,
    pub items: Vec<(Option<String>, Statement)>,
}

/// input_declaration ::=
/// input ( net_type )? ( signed )? ( range )? list_of_port_identifiers
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Input {
    pub name: String,
    pub net_type: Option<NetType>,
    pub width: Option<u32>,
    pub is_signed: bool,
}

// Архитектура
// Грамматика
// Выбор парсера
// Что транслятору можно подать на вход и как он реагирует на ошибки
//

/// inout_declaration ::=
/// inout ( <net_type> )? ( signed )? ( <range> )? <list_of_port_identifiers>
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Inout {
    pub name: String,
    pub net_type: Option<NetType>,
    pub width: Option<u32>,
    pub is_signed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegNetType {
    NetType(NetType),
    Reg(bool),
}

/// output_declaration ::=
/// output ( net_type )? ( signed )? ( range )? list_of_port_identifiers
/// |
/// output ( reg )? ( signed )? ( range )? list_of_port_identifiers
/// |
/// output reg ( signed )? ( range )? list_of_variable_port_identifiers
/// |
/// output ( output_variable_type )? list_of_port_identifiers
/// |
/// output output_variable_type list_of_variable_port_identifiers
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Output {
    pub name: String,
    pub reg_net_type: Option<RegNetType>,
    pub width: Option<u32>,
    pub is_signed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OperationType {
    Sync,  // <=
    Async, // =
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assignment {
    pub name: String,
    pub ass_type: OperationType,
    pub right: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Module {
    pub name: String,
    pub statements: Vec<Statement>,
    pub ports: Vec<Port>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// TODO: заменить на NetType. Wire-это частное
pub struct Wire {
    pub name: String,
    pub width: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Register {
    pub name: String,
    pub width: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalParam {
    pub name: String,
    pub value: Number,
    pub width: u32,
    pub is_signed: bool,
}

// number ::=
// decimal_number
// | octal_number
// | binary_number
// | hex_number
// | real_number real_number

#[derive(Clone, Debug, PartialEq, Eq)]
/// TODO: real_number
pub enum Number {
    Binary(u32, String),
    Octal(u32, String),
    Decimal(u32, String),
    Hex(u32, String),
}
