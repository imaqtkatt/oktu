use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub enum TypeKind {
  Variable { name: String },
  Generalized { id: usize },
  Hole { hole: Hole },
  Arrow { t1: Rc<TypeKind>, t2: Rc<TypeKind> },
  Enum { name: String },
  Tuple { elements: Vec<Type> },
  Number,
  String,
  Boolean,
  Error,
}

#[macro_export]
macro_rules! arr {
  ($t1:expr => $t2:expr) => {
    TypeKind::Arrow { t1: $t1.into(), t2: $t2.into() }
  };
}

pub type Type = Rc<TypeKind>;

#[derive(Debug, Clone)]
pub enum HoleKind {
  Bound { t: Type },
  Unbound { name: String, level: usize },
}

#[derive(Clone, Debug)]
pub struct Hole(Rc<RefCell<HoleKind>>);

impl PartialEq for Hole {
  fn eq(&self, other: &Self) -> bool {
    Rc::ptr_eq(&self.0, &other.0)
  }
}

impl Eq for Hole {}

#[derive(Debug, Clone)]
pub struct Scheme {
  pub binds: Vec<String>,
  pub t: Type,
}

impl Hole {
  pub fn new(name: String, level: usize) -> Self {
    Self(Rc::new(RefCell::new(HoleKind::Unbound { name, level })))
  }

  pub fn get(&self) -> HoleKind {
    self.0.borrow().clone()
  }

  pub fn get_mut(&self) -> std::cell::RefMut<'_, HoleKind> {
    self.0.borrow_mut()
  }

  pub fn fill(&self, t: Type) {
    *self.0.borrow_mut() = HoleKind::Bound { t }
  }
}

impl TypeKind {
  pub fn instantiate(self: Type, substitutions: &[Type]) -> Type {
    match &&*self {
      TypeKind::Variable { .. } => self.clone(),
      TypeKind::Generalized { id } => substitutions[*id].clone(),
      TypeKind::Hole { hole } => match hole.get() {
        HoleKind::Bound { t } => t.instantiate(substitutions),
        HoleKind::Unbound { .. } => self.clone(),
      },
      TypeKind::Arrow { t1, t2 } => {
        let t1 = t1.clone().instantiate(substitutions);
        let t2 = t2.clone().instantiate(substitutions);
        Type::new(TypeKind::Arrow { t1, t2 })
      }
      TypeKind::Enum { .. } => self.clone(),
      TypeKind::Tuple { elements } => Type::new(TypeKind::Tuple {
        elements: elements.iter().map(|e| e.clone().instantiate(substitutions)).collect::<Vec<_>>(),
      }),
      TypeKind::Number => self.clone(),
      TypeKind::String => self.clone(),
      TypeKind::Boolean => self.clone(),
      TypeKind::Error => self.clone(),
    }
  }

  pub fn number() -> Type {
    Type::new(TypeKind::Number)
  }

  pub fn string() -> Type {
    Type::new(TypeKind::String)
  }

  pub fn boolean() -> Type {
    Type::new(TypeKind::Boolean)
  }

  pub fn num_num() -> Type {
    arr!(TypeKind::Number => arr!(TypeKind::Number => TypeKind::Number)).into()
  }

  pub fn num_logical() -> Type {
    arr!(TypeKind::Number => arr!(TypeKind::Number => TypeKind::Boolean)).into()
  }

  pub fn str_str() -> Type {
    arr!(TypeKind::String => arr!(TypeKind::String => TypeKind::String)).into()
  }

  /// string -> 'a -> 'a
  pub fn print_string() -> Type {
    arr!(TypeKind::String => arr!(TypeKind::Generalized { id: 0 } => TypeKind::Generalized { id: 0 })).into()
  }

  /// number -> 'a
  pub fn exit() -> Type {
    arr!(TypeKind::Number => TypeKind::Generalized { id: 0 }).into()
  }
}

impl Scheme {
  pub fn new(binds: Vec<String>, t: Type) -> Self {
    Self { binds, t }
  }

  pub fn skolemize(&self) -> Type {
    let mut substitutions = Vec::new();

    for bind in self.binds.iter() {
      substitutions.push(Type::new(TypeKind::Variable { name: bind.clone() }));
    }

    self.t.clone().instantiate(&substitutions)
  }
}
