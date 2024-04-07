use crate::{checker::TypeKind, report::Diagnostic};

use super::{Env, Hole, HoleKind, Type};

pub struct UnifyError(Type, Type);

pub struct OccursCheck(Hole, Type);

fn occurs(hole: Hole, t: Type) -> bool {
  match &*t {
    TypeKind::Variable { .. } => false,
    TypeKind::Generalized { .. } => false,
    TypeKind::Hole { hole: this_hole } => this_hole.clone() == hole,
    TypeKind::Arrow { t1, t2 } => occurs(hole.clone(), t1.clone()) || occurs(hole, t2.clone()),
    TypeKind::Enum { .. } => false,
    TypeKind::Number => false,
    TypeKind::String => false,
    TypeKind::Boolean => false,
    TypeKind::Error => false,
  }
}

pub fn unify(env: &Env, t1: Type, t2: Type) -> bool {
  use TypeKind::*;
  match (&*t1, &*t2) {
    (Variable { name: x }, Variable { name: y }) => x == y,
    (Generalized { id: x }, Generalized { id: y }) => x == y,
    (Hole { hole }, _) => unify_hole(env, hole.clone(), t2.clone(), false),
    (_, Hole { hole }) => unify_hole(env, hole.clone(), t1.clone(), true),
    (Arrow { t1: a, t2: b }, Arrow { t1: c, t2: d }) => {
      unify(env, a.clone(), c.clone()) && unify(env, b.clone(), d.clone())
    }
    (Enum { name: x }, Enum { name: y }) => x == y,
    (Number, Number) => true,
    (String, String) => true,
    (Boolean, Boolean) => true,
    (_, _) => {
      env.reporter.report(UnifyError(t1, t2));
      false
    }
  }
}

fn unify_hole(env: &Env, hole: Hole, t: Type, swap: bool) -> bool {
  match hole.get() {
    HoleKind::Bound { t: hole_type } => {
      if swap {
        unify(env, t, hole_type)
      } else {
        unify(env, hole_type, t)
      }
    }
    HoleKind::Unbound { .. } => {
      if occurs(hole.clone(), t.clone()) {
        println!("occurs between {hole:?} and {t}");
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
    format!("Type mismatch between {} and {}.", self.0, self.1)
  }

  fn severity(&self) -> crate::report::Severity {
    crate::report::Severity::Error
  }

  fn extra(&self) -> Vec<String> {
    vec![]
  }
}

impl Diagnostic for OccursCheck {
  fn message(&self) -> String {
    format!("Occurs check between {} and {}", self.0, self.1)
  }

  fn severity(&self) -> crate::report::Severity {
    crate::report::Severity::Error
  }

  fn extra(&self) -> Vec<String> {
    vec![]
  }
}
