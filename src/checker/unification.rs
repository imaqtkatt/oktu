use crate::checker::TypeKind;

use super::{Hole, HoleKind, Type};

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

// TODO: not panic
pub fn unify(t1: Type, t2: Type) -> bool {
  use TypeKind::*;
  match (&*t1, &*t2) {
    (Variable { name: x }, Variable { name: y }) => x == y,
    (Generalized { id: x }, Generalized { id: y }) => x == y,
    (Hole { hole }, _) => unify_hole(hole.clone(), t2.clone(), false),
    (_, Hole { hole }) => unify_hole(hole.clone(), t1.clone(), true),
    (Arrow { t1: a, t2: b }, Arrow { t1: c, t2: d }) => {
      unify(a.clone(), c.clone()) && unify(b.clone(), d.clone())
    }
    (Enum { name: x }, Enum { name: y }) => x == y,
    (Number, Number) => true,
    (String, String) => true,
    (Boolean, Boolean) => true,
    x => panic!("{x:?}"),
  }
}

// TODO: not panic
fn unify_hole(hole: Hole, t: Type, swap: bool) -> bool {
  match hole.get() {
    HoleKind::Bound { t: hole_type } => {
      if swap {
        unify(t, hole_type)
      } else {
        unify(hole_type, t)
      }
    }
    HoleKind::Unbound { .. } => {
      if occurs(hole.clone(), t.clone()) {
        panic!("occurs between {hole:?} and {t}")
      } else {
        hole.fill(t);
        true
      }
    }
  }
}
