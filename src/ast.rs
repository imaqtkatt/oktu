use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Src(pub(crate) Range<usize>);

impl Src {
  pub fn new(start: usize, end: usize) -> Self {
    Self(start..end)
  }
}

#[derive(Clone, Debug)]
pub enum Literal {
  Number { value: i32 },
  String { value: String },
  Boolean { value: bool },
}

#[derive(Clone, Debug)]
pub enum Expression {
  /// ?name
  Hole { name: String, src: Src },
  /// a..z | _
  Variable { name: String, src: Src },
  /// fun var -> body
  Fun { variable: String, body: Box<Expression>, src: Src },
  /// f x
  Application { function: Box<Expression>, argument: Box<Expression>, src: Src },
  /// num | str | bool
  Literal { literal: Literal, src: Src },
  /// let bind = value in next
  Let { bind: String, value: Box<Expression>, next: Box<Expression>, src: Src },
  /// if condition then expr else expr
  If { condition: Box<Expression>, then: Box<Expression>, otherwise: Box<Expression>, src: Src },
  /// match x with
  ///   pat => body,
  ///   pat => body,
  /// end
  Match { scrutinee: Box<Expression>, arms: Vec<Arm>, src: Src },
  /// lhs op rhs
  BinaryOp { op: Operation, lhs: Box<Expression>, rhs: Box<Expression>, src: Src },
  /// .variant
  Variant { variant: String, src: Src },
  /// (...,)
  Tuple { elements: Vec<Expression>, src: Src },
}

impl Expression {
  pub fn src(&self) -> Src {
    match self {
      Expression::Hole { src, .. } => src.clone(),
      Expression::Variable { src, .. } => src.clone(),
      Expression::Fun { src, .. } => src.clone(),
      Expression::Application { src, .. } => src.clone(),
      Expression::Literal { src, .. } => src.clone(),
      Expression::Let { src, .. } => src.clone(),
      Expression::If { src, .. } => src.clone(),
      Expression::Match { src, .. } => src.clone(),
      Expression::BinaryOp { src, .. } => src.clone(),
      Expression::Variant { src, .. } => src.clone(),
      Expression::Tuple { src, .. } => src.clone(),
    }
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
  Variable { name: String, src: Src },
  Variant { variant: String, src: Src },
  Literal { literal: Literal, src: Src },
  Tuple { binds: Vec<String>, src: Src },
}

impl Pattern {
  pub fn src(&self) -> Src {
    match self {
      Pattern::Variable { src, .. } => src.clone(),
      Pattern::Variant { src, .. } => src.clone(),
      Pattern::Literal { src, .. } => src.clone(),
      Pattern::Tuple { src, .. } => src.clone(),
    }
  }
}

pub type Parameters = Vec<String>;

#[derive(Clone, Debug)]
pub struct Function {
  pub name: String,
  pub rec: bool,
  pub parameters: Parameters,
  pub body: Expression,
}

#[derive(Clone, Debug)]
pub struct Enum {
  pub name: String,
  pub parameters: Vec<String>,
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

impl Program {
  pub fn empty() -> Self {
    Self { file_name: None, declarations: Vec::new() }
  }

  pub fn new(declarations: Vec<TopLevel>) -> Self {
    Self { file_name: None, declarations }
  }
}
