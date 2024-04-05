use crate::{
  ast::Literal,
  checker::{Env, Type, TypeKind},
  elab,
};

use super::Infer;

impl Infer for Literal {
  type Out = elab::Literal;

  fn infer(self, _: Env) -> (Self::Out, Type) {
    match self {
      Literal::Number { value } => (elab::Literal::Number { value }, TypeKind::number()),
      Literal::String { value } => (elab::Literal::String { value }, TypeKind::string()),
      Literal::Boolean { value } => (elab::Literal::Boolean { value }, TypeKind::boolean()),
    }
  }
}
