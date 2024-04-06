use core::fmt;

use super::{Arm, Enum, Expression, Function, Literal, Operation, Pattern, Program, TopLevel};

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
      Operation::Add => write!(f, "+"),
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

impl fmt::Display for Pattern {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Pattern::Error { message } => write!(f, "<Error: \"{message}\">"),
      Pattern::Wildcard => write!(f, "_"),
      Pattern::Variable { name } => write!(f, "{name}"),
      Pattern::Variant { variant: name } => write!(f, ".{name}"),
      Pattern::Literal { literal } => write!(f, "{literal}"),
    }
  }
}

impl fmt::Display for Arm {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} => {}", self.left, self.right)
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
      Expression::If { condition, then, otherwise } => {
        write!(f, "if {condition} then {then} else {otherwise}")
      }
      Expression::Match { scrutinee, arms } => {
        write!(f, "match {scrutinee} with ")?;
        for arm in arms {
          write!(f, "{arm}, ")?;
        }
        write!(f, "end")?;
        Ok(())
      }
      Expression::BinaryOp { op, lhs, rhs } => write!(f, "({lhs} {op} {rhs})"),
      Expression::Variant { variant } => write!(f, ".{variant}"),
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

impl fmt::Display for Program {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(file_name) = &self.file_name {
      write!(f, "// {}\n\n", file_name)?;
    }
    for decl in self.declarations.iter() {
      write!(f, "{}\n\n", decl)?;
    }

    Ok(())
  }
}
