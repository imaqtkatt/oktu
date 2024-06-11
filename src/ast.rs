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
pub enum ExpressionType {
  /// "()"
  Unit,
  /// ?name
  Hole { name: String },
  /// a..z | _
  Variable { name: String },
  /// fun var -> body
  Fun {
    variable: String,
    body: Box<Expression>,
  },
  /// f x
  Application {
    function: Box<Expression>,
    argument: Box<Expression>,
  },
  /// num | str | bool
  Literal { literal: Literal },
  /// let bind = value in next
  Let {
    bind: String,
    value: Box<Expression>,
    next: Box<Expression>,
  },
  /// if condition then expr else expr
  If {
    condition: Box<Expression>,
    then: Box<Expression>,
    otherwise: Box<Expression>,
  },
  /// match x with
  ///   pat => body,
  ///   pat => body,
  /// end
  Match {
    scrutinee: Box<Expression>,
    arms: Vec<Arm>,
  },
  /// lhs op rhs
  BinaryOp {
    op: Operation,
    lhs: Box<Expression>,
    rhs: Box<Expression>,
  },
  /// .variant
  Variant { variant: String },
  /// (...,)
  Tuple { elements: Vec<Expression> },
}

#[derive(Clone, Debug)]
pub struct Expression {
  pub data: Box<ExpressionType>,
  pub src: Src,
}

impl Expression {
  pub fn new(data: ExpressionType, src: Src) -> Self {
    Self {
      data: Box::new(data),
      src,
    }
  }

  pub fn src(&self) -> Src {
    self.src.clone()
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
    Self {
      file_name: None,
      declarations: Vec::new(),
    }
  }

  pub fn new(declarations: Vec<TopLevel>) -> Self {
    Self {
      file_name: None,
      declarations,
    }
  }
}
