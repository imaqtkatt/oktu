use core::fmt;

#[derive(Clone, Debug)]
pub enum Literal {
  Number { value: i32 },
  String { value: String },
  Boolean { value: bool },
}

#[derive(Clone, Debug)]
pub enum Expression {
  /// For type error.
  Error {
    message: String,
  },
  Hole {
    name: String,
  },
  /// a..z | _
  Variable {
    name: String,
  },
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
  Literal {
    literal: Literal,
  },
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
  /// match x {
  ///   pat => body,
  ///   pat => body,
  /// }
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
  Enum {
    variant: String,
  },
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
  Enum { name: String },
  Literal { literal: Literal },
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
  // pub parameters: Vec<String>,
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
  pub file_name: String,
  pub declarations: Vec<TopLevel>,
}

impl fmt::Display for Literal {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Literal::Number { value } => write!(f, "{value}"),
      Literal::String { value } => write!(f, "{value:?}"),
      Literal::Boolean { value } => write!(f, "{value}"),
    }
  }
}

impl fmt::Display for Operation {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Operation::Add => todo!(),
      Operation::Sub => write!(f, "-"),
      Operation::Mul => todo!(),
      Operation::Div => todo!(),
      Operation::Gt => write!(f, ">"),
      Operation::Gte => todo!(),
      Operation::Lt => todo!(),
      Operation::Lte => todo!(),
      Operation::Eq => todo!(),
      Operation::Neq => todo!(),
      Operation::Concat => write!(f, "++"),
    }
  }
}

impl fmt::Display for Expression {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Expression::Error { message } => write!(f, "<Error: \"{message}\">"),
      Expression::Hole { name } => write!(f, "?{name}"),
      Expression::Variable { name } => write!(f, "{name}"),
      Expression::Fun { variable, body } => write!(f, "fun {variable} -> {body}"),
      Expression::Application { function, argument } => write!(f, "({function} {argument})"),
      Expression::Literal { literal } => write!(f, "{literal}"),
      Expression::Let { bind, value, next } => write!(f, "let {bind} = {value} in {next}"),
      Expression::If {
        condition,
        then,
        otherwise,
      } => write!(f, "if {condition} then {then} else {otherwise}"),
      Expression::Match { scrutinee, arms: _ } => write!(f, "match {scrutinee} with {{ .. }}"),
      Expression::BinaryOp { op, lhs, rhs } => write!(f, "({lhs} {op} {rhs})"),
      Expression::Enum { variant } => write!(f, ".{variant}"),
    }
  }
}

impl fmt::Display for Function {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "let ")?;
    if self.rec {
      write!(f, "rec ")?;
    }
    write!(f, "{} := ", self.name)?;
    write!(f, "{}", self.body)?;
    Ok(())
  }
}

impl fmt::Display for Enum {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "enum ")?;
    write!(f, "{} ", self.name)?;
    write!(f, " := ")?;
    for variant in &self.variants {
      write!(f, ".{variant}, ")?;
    }
    Ok(())
  }
}

impl fmt::Display for TopLevel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TopLevel::Function(function) => write!(f, "{function}"),
      TopLevel::Enum(r#enum) => write!(f, "{}", r#enum),
    }
  }
}
