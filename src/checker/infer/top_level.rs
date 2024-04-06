use crate::{
  ast::{Enum, Function, TopLevel},
  checker::{Env, Scheme, Type, TypeKind},
  elab,
};

use super::Infer;

impl Infer for TopLevel {
  type Out = (Env, elab::TopLevel);

  // Returns the updated env with the infered top level definition
  fn infer(self, mut env: Env) -> (Self::Out, Type) {
    match self {
      TopLevel::Function(Function { name: function_name, rec, parameters, body }) => {
        let mut new_env = env.clone();
        if rec {
          let rec_hole = new_env.new_hole();
          new_env.variables.insert(function_name.clone(), Scheme::new(vec![], rec_hole));
        }
        let mut parameter_types = Vec::new();

        new_env.enter_level();
        for param in parameters.iter() {
          let hole = new_env.new_hole();
          new_env.variables.insert(param.clone(), Scheme::new(vec![], hole.clone()));
          parameter_types.push(hole);
        }

        let (elab_body, body_type) = body.infer(new_env.clone());
        new_env.leave_level();

        let function_type = parameter_types
          .into_iter()
          .rfold(body_type, |acc, param| Type::new(TypeKind::Arrow { t1: param, t2: acc }));

        env.let_decls.insert(function_name.clone(), new_env.generalize(function_type.clone()));

        let elab_body = parameters.into_iter().rfold(elab_body, |acc, param| {
          elab::Expression::Fun { variable: param, body: Box::new(acc) }
        });

        let elab =
          elab::TopLevel::Function(elab::Function { name: function_name, rec, body: elab_body });

        ((env, elab), function_type)
      }
      TopLevel::Enum(Enum { name: enum_name, parameters, variants }) => {
        env.enum_decls.insert(enum_name.clone(), variants.len());

        let scheme = parameters.clone();
        let vars = scheme
          .iter()
          .enumerate()
          .map(|(id, _)| Type::new(TypeKind::Generalized { id }))
          .collect::<Vec<_>>();

        for (name, generalized) in scheme.iter().zip(vars.clone()) {
          env.type_variables.insert(name.clone(), generalized);
        }

        for variant in variants.iter() {
          env.variant_to_enum.insert(variant.clone(), enum_name.clone());
        }

        let elab = elab::TopLevel::Enum(elab::Enum { name: enum_name.clone(), variants });

        ((env, elab), Type::new(TypeKind::Enum { name: enum_name }))
      }
    }
  }
}
