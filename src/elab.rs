pub mod display;

#[derive(Clone, Debug)]
pub enum Literal {
  Number { value: i32 },
  String { value: String },
  Boolean { value: bool },
}

#[derive(Clone, Debug)]
pub enum Expression {
  /// For type error.
  Error { message: String },
  /// ?name
  Hole { name: String },
  /// a..z | _
  Variable { name: String },
  /// fun var -> body
  Fun { variable: String, body: Box<Expression> },
  /// f x
  Application { function: Box<Expression>, argument: Box<Expression> },
  /// num | str | bool
  Literal { literal: Literal },
  /// let bind = value in next
  Let { bind: String, value: Box<Expression>, next: Box<Expression> },
  /// if condition then expr else expr
  If { condition: Box<Expression>, then: Box<Expression>, otherwise: Box<Expression> },
  /// match x with
  ///   pat => body,
  ///   pat => body,
  /// end
  Match { scrutinee: Box<Expression>, arms: Vec<Arm> },
  /// lhs op rhs
  BinaryOp { op: Operation, lhs: Box<Expression>, rhs: Box<Expression> },
  /// .variant
  Variant { variant: String },
  /// (...,)
  Tuple { elements: Vec<Expression> },
}

impl Expression {
  pub fn error(message: String) -> Self {
    Self::Error { message }
  }
}

#[derive(Clone, Debug)]
pub enum Operation {
  Add,
  Sub,
  Mul,
  Div,
  Gt,
  Gte,
  Lt,
  Lte,
  Eq,
  Neq,
  Concat,
}

#[derive(Clone, Debug)]
pub struct Arm {
  pub left: Pattern,
  pub right: Expression,
}

#[derive(Clone, Debug)]
pub enum Pattern {
  Error { message: String },
  Wildcard,
  Variable { name: String },
  Variant { variant: String },
  Literal { literal: Literal },
  Tuple { binds: Vec<String> },
}

impl Pattern {
  pub fn error(message: String) -> Self {
    Self::Error { message }
  }
}

#[derive(Clone, Debug)]
pub struct Function {
  pub name: String,
  pub rec: bool,
  pub body: Expression,
}

#[derive(Clone, Debug)]
pub struct Enum {
  pub name: String,
  pub variants: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum TopLevel {
  Function(Function),
  Enum(Enum),
}

#[derive(Clone, Debug)]
pub struct Program {
  pub file_name: Option<Box<str>>,
  pub declarations: Vec<TopLevel>,
}
