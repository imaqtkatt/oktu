use core::fmt;

use super::{HoleKind, TypeKind};

impl fmt::Display for TypeKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      TypeKind::Variable { name } => write!(f, "{name}"),
      TypeKind::Generalized { id } => {
        let c = std::char::from_u32(*id as u32 + 97).unwrap_or('?');
        write!(f, "{c}")
      }
      TypeKind::Hole { hole } => match hole.get() {
        HoleKind::Bound { t } => write!(f, "{t}"),
        HoleKind::Unbound { name, level } => write!(f, "^{name}_{level}"),
      },
      TypeKind::Arrow { t1, t2 } => {
        if t1.need_parens() {
          write!(f, "({t1}) -> {t2}")
        } else {
          write!(f, "{t1} -> {t2}")
        }
      }
      TypeKind::Enum { name } => write!(f, "{name}"),
      TypeKind::Number => write!(f, "number"),
      TypeKind::String => write!(f, "string"),
      TypeKind::Boolean => write!(f, "bool"),
      TypeKind::Error => write!(f, "<Error>"),
    }
  }
}
