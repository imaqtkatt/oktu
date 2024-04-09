use crate::{
  arr,
  ast::{self, Expression},
  checker::{unification::unify, Env, Scheme, Type, TypeKind},
  elab,
  report::Diagnostic,
};

use super::Infer;

enum ExpressionInferError {
  UnboundVariable(String),
  UnknownVariant(String),
}

impl Infer for Expression {
  type Out = elab::Expression;

  fn infer(self, mut env: Env) -> (Self::Out, Type) {
    match self {
      Expression::Hole { name } => {
        (elab::Expression::Hole { name: name.clone() }, env.new_hole_named(name))
      }
      Expression::Variable { name } => match env.fetch(&name) {
        Some(scheme) => (elab::Expression::Variable { name }, env.instantiate(scheme.clone())),
        None => {
          env.reporter.report(ExpressionInferError::UnboundVariable(name.clone()));
          (
            elab::Expression::error(format!("Unbound variable '{name}'.")),
            Type::new(TypeKind::Error),
          )
        }
      },
      Expression::Fun { variable, body } => {
        let hole = env.new_hole();
        let scheme = Scheme::new(vec![], hole.clone());

        let mut new_env = env.clone();
        new_env.variables.insert(variable.clone(), scheme);

        let (elab_body, body_type) = body.infer(new_env);
        let arrow = Type::new(arr!(hole => body_type));

        (elab::Expression::Fun { variable, body: Box::new(elab_body) }, arrow)
      }
      Expression::Application { function, argument } => {
        let (elab_function, function_type) = function.infer(env.clone());
        let (elab_argument, argument_type) = argument.infer(env.clone());

        let hole = env.new_hole();

        let arrow_type: Type = arr!(argument_type => hole.clone()).into();

        unify(&env, function_type, arrow_type.clone());
        (
          elab::Expression::Application {
            function: Box::new(elab_function),
            argument: Box::new(elab_argument),
          },
          hole,
        )
      }
      Expression::Literal { literal } => {
        let (elab_literal, literal_type) = literal.infer(env);
        (elab::Expression::Literal { literal: elab_literal }, literal_type)
      }
      Expression::Let { bind, value, next } => {
        env.enter_level();
        let (elab_value, value_type) = value.infer(env.clone());
        env.leave_level();

        let value_g = env.generalize(value_type);

        let mut new_env = env.clone();
        new_env.variables.insert(bind.clone(), value_g);

        let (elab_next, next_type) = next.infer(new_env);
        (
          elab::Expression::Let { bind, value: Box::new(elab_value), next: Box::new(elab_next) },
          next_type,
        )
      }
      Expression::If { condition, then, otherwise } => {
        let (elab_condition, condition_type) = condition.infer(env.clone());
        unify(&env, condition_type, TypeKind::boolean());

        let return_type = env.new_hole();

        let (elab_then, then_type) = then.infer(env.clone());
        unify(&env, return_type.clone(), then_type);

        let (elab_otherwise, otherwise_type) = otherwise.infer(env.clone());
        unify(&env, return_type.clone(), otherwise_type);

        (
          elab::Expression::If {
            condition: Box::new(elab_condition),
            then: Box::new(elab_then),
            otherwise: Box::new(elab_otherwise),
          },
          return_type,
        )
      }
      Expression::Match { scrutinee, arms } => {
        let return_type = env.new_hole();

        let (elab_scrutinee, scrutinee_type) = scrutinee.infer(env.clone());
        let mut elab_arms = Vec::new();

        for ast::Arm { left, right } in arms {
          let ((binds, elab_left), left_type) = left.infer(env.clone());
          for (bind, value) in binds {
            env.variables.insert(bind, Scheme::new(vec![], value));
          }

          let (elab_right, right_type) = right.infer(env.clone());

          unify(&env, left_type, scrutinee_type.clone());
          unify(&env, return_type.clone(), right_type);

          elab_arms.push(elab::Arm { left: elab_left, right: elab_right })
        }

        (
          elab::Expression::Match { scrutinee: Box::new(elab_scrutinee), arms: elab_arms },
          return_type,
        )
      }
      Expression::BinaryOp { op, lhs, rhs } => {
        let (elab_op, op_type) = op.infer(env.clone());

        let (elab_lhs, lhs_type) = lhs.infer(env.clone());
        let (elab_rhs, rhs_type) = rhs.infer(env.clone());

        let ret_type = env.new_hole();
        let to_unify: Type = arr!(lhs_type => arr!(rhs_type => ret_type.clone())).into();

        unify(&env, to_unify, op_type);
        (
          elab::Expression::BinaryOp {
            op: elab_op,
            lhs: Box::new(elab_lhs),
            rhs: Box::new(elab_rhs),
          },
          ret_type,
        )
      }
      Expression::Variant { variant } => match env.variant_to_enum.get(&variant) {
        Some(name) => {
          (elab::Expression::Variant { variant }, Type::new(TypeKind::Enum { name: name.clone() }))
        }
        None => {
          env.reporter.report(ExpressionInferError::UnknownVariant(variant.clone()));
          (
            elab::Expression::error(format!("Unknown variant '{variant}'.")),
            Type::new(TypeKind::Error),
          )
        }
      },
      Expression::Tuple { elements } => {
        let (elab_elements, elements_type) =
          elements.into_iter().map(|e| e.infer(env.clone())).unzip();
        (
          elab::Expression::Tuple { elements: elab_elements },
          Type::new(TypeKind::Tuple { elements: elements_type }),
        )
      }
    }
  }
}

impl Diagnostic for ExpressionInferError {
  fn message(&self) -> String {
    match self {
      ExpressionInferError::UnboundVariable(name) => format!("Unbound variable '{name}'."),
      ExpressionInferError::UnknownVariant(variant) => format!("Unknown variant '{variant}'."),
    }
  }

  fn severity(&self) -> crate::report::Severity {
    crate::report::Severity::Error
  }

  fn extra(&self) -> Vec<String> {
    vec![]
  }
}
