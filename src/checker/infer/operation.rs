use crate::{
  ast::Operation,
  checker::{Env, Type, TypeKind},
  elab,
};

use super::Infer;

impl Infer for Operation {
  type Out = elab::Operation;

  fn infer(self, _: Env) -> (Self::Out, Type) {
    match self {
      Operation::Add => (elab::Operation::Add, TypeKind::num_num()),
      Operation::Sub => (elab::Operation::Sub, TypeKind::num_num()),
      Operation::Mul => (elab::Operation::Mul, TypeKind::num_num()),
      Operation::Div => (elab::Operation::Div, TypeKind::num_num()),
      Operation::Gt => (elab::Operation::Gt, TypeKind::num_logical()),
      Operation::Gte => (elab::Operation::Gte, TypeKind::num_logical()),
      Operation::Lt => (elab::Operation::Lt, TypeKind::num_logical()),
      Operation::Lte => (elab::Operation::Lte, TypeKind::num_logical()),
      Operation::Eq => (elab::Operation::Eq, TypeKind::num_logical()),
      Operation::Neq => (elab::Operation::Neq, TypeKind::num_logical()),
      Operation::Concat => (elab::Operation::Concat, TypeKind::str_str()),
    }
  }
}
