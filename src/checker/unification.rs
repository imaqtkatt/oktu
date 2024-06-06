use crate::{ast::Src, checker::TypeKind, report::Diagnostic};

use super::{Env, Hole, HoleKind, Type};

pub struct UnifyError(Type, Type, Src);

pub struct OccursCheck(Src);

fn occurs(hole: Hole, t: Type) -> bool {
  match &*t {
    TypeKind::Variable { .. } => false,
    TypeKind::Generalized { .. } => false,
    TypeKind::Hole { hole: this_hole } => this_hole.clone() == hole,
    TypeKind::Arrow { t1, t2 } => occurs(hole.clone(), t1.clone()) || occurs(hole, t2.clone()),
    TypeKind::Enum { .. } => false,
    TypeKind::Tuple { elements } => elements.iter().any(|e| occurs(hole.clone(), e.clone())),
    TypeKind::Number => false,
    TypeKind::String => false,
    TypeKind::Boolean => false,
    TypeKind::Error => false,
  }
}

pub fn unify(env: &Env, t1: Type, t2: Type, src: Src) -> bool {
  use TypeKind::*;
  match (&*t1, &*t2) {
    (Variable { name: x }, Variable { name: y }) => x == y,

    (Generalized { id: x }, Generalized { id: y }) => x == y,

    (Hole { hole }, _) => unify_hole(env, hole.clone(), t2.clone(), false, src),
    (_, Hole { hole }) => unify_hole(env, hole.clone(), t1.clone(), true, src),

    (Arrow { t1: a, t2: b }, Arrow { t1: c, t2: d }) => {
      unify(env, a.clone(), c.clone(), src.clone()) && unify(env, b.clone(), d.clone(), src)
    }

    (Enum { name: x }, Enum { name: y }) => x == y,

    (Number, Number) => true,
    (String, String) => true,
    (Boolean, Boolean) => true,

    (Tuple { elements: x }, Tuple { elements: y }) if x.len() == y.len() => {
      x.iter().zip(y.iter()).all(|(a, b)| unify(env, a.clone(), b.clone(), src.clone()))
    }

    (_, _) => {
      env.reporter.report(UnifyError(t1, t2, src));
      false
    }
  }
}

fn unify_hole(env: &Env, hole: Hole, t: Type, swap: bool, src: Src) -> bool {
  match hole.get() {
    HoleKind::Bound { t: hole_type } => {
      if swap {
        unify(env, t, hole_type, src)
      } else {
        unify(env, hole_type, t, src)
      }
    }
    HoleKind::Unbound { .. } => {
      if occurs(hole.clone(), t.clone()) {
        env.reporter.report(OccursCheck(src));
        false
      } else {
        hole.fill(t);
        true
      }
    }
  }
}

impl Diagnostic for UnifyError {
  fn message(&self) -> String {
    format!("Type mismatch.")
  }

  fn severity(&self) -> crate::report::Severity {
    crate::report::Severity::Error
  }

  fn extra(&self) -> Vec<String> {
    vec![format!("Expected {} but got {}.", self.0, self.1)]
  }

  fn src(&self) -> Option<crate::ast::Src> {
    Some(self.2.clone())
  }
}

impl Diagnostic for OccursCheck {
  fn message(&self) -> String {
    format!("Occurs check.")
  }

  fn severity(&self) -> crate::report::Severity {
    crate::report::Severity::Error
  }

  fn extra(&self) -> Vec<String> {
    vec![]
  }

  fn src(&self) -> Option<crate::ast::Src> {
    Some(self.0.clone())
  }
}
