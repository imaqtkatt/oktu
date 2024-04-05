pub mod expression;
pub mod literal;
pub mod operation;
pub mod pattern;
pub mod top_level;

use super::{Env, Type};

pub trait Infer {
  type Out;

  fn infer(self, env: Env) -> (Self::Out, Type);
}
