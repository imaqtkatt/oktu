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
        if name.starts_with('_') {
          ((map, elab::Pattern::Wildcard), env.new_hole())
        } else {
          let hole = env.new_hole();
          map.insert(name.clone(), hole.clone());
          ((map, elab::Pattern::Variable { name }), hole)
        }
      }
      Pattern::Variant { variant } => match env.variant_to_enum.get(&variant) {
        Some(enum_name) => (
          (map, elab::Pattern::Variant { variant }),
          Type::new(TypeKind::Enum { name: enum_name.clone() }),
        ),
        None => (
          (map, elab::Pattern::error(format!("Unknown variant '{variant}'."))),
          Type::new(TypeKind::Error),
        ),
      },
      Pattern::Literal { literal } => {
        let (elab_literal, literal_type) = literal.infer(env);
        ((map, elab::Pattern::Literal { literal: elab_literal }), literal_type)
      }
      Pattern::Tuple { binds } => {
        let mut elab_binds = Vec::new();
        let mut elements = Vec::new();

        for bind in binds.iter() {
          let hole = env.new_hole();
          elab_binds.push(bind.clone());
          map.insert(bind.clone(), hole.clone());
          elements.push(hole);
        }

        ((map, elab::Pattern::Tuple { binds: elab_binds }), Type::new(TypeKind::Tuple { elements }))
      }
    }
  }
}
