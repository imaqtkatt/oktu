use std::collections::HashMap;

use crate::{
  ast::Pattern,
  checker::{Env, Type, TypeKind},
  elab,
};

use super::Infer;

impl Infer for Pattern {
  type Out = (HashMap<String, Type>, elab::Pattern);

  fn infer(self, mut env: Env) -> (Self::Out, Type) {
    let mut map = HashMap::new();
    match self {
      Pattern::Variable { name } => {
        if name.starts_with("_") {
          ((map, elab::Pattern::Wildcard), env.new_hole())
        } else {
          let hole = env.new_hole();
          map.insert(name.clone(), hole.clone());
          ((map, elab::Pattern::Variable { name }), hole)
        }
      }
      Pattern::Enum { name } => match env.variant_to_enum.get(&name) {
        Some(enum_name) => (
          (map, elab::Pattern::Enum { name }),
          Type::new(TypeKind::Enum {
            name: enum_name.clone(),
          }),
        ),
        None => (
          (
            map,
            elab::Pattern::error(format!("Unknown variant '{name}'.")),
          ),
          Type::new(TypeKind::Error),
        ),
      },
      Pattern::Literal { literal } => {
        let (elab_literal, literal_type) = literal.infer(env);
        (
          (
            map,
            elab::Pattern::Literal {
              literal: elab_literal,
            },
          ),
          literal_type,
        )
      }
    }
  }
}
