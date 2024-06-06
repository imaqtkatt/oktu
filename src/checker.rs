pub mod display;
pub mod infer;
pub mod types;
pub mod unification;

use std::collections::HashMap;

use crate::{ast::Program, elab, report::Reporter};

use self::{infer::Infer, types::*};

#[derive(Clone)]
pub struct Env {
  pub variables: HashMap<String, Scheme>,
  pub type_variables: HashMap<String, Type>,
  pub let_decls: HashMap<String, Scheme>,
  pub enum_decls: HashMap<String, usize>,
  pub variant_to_enum: HashMap<String, String>,
  pub level: usize,
  pub counter: usize,
  pub reporter: Reporter,
}

impl Env {
  pub fn new(reporter: Reporter) -> Self {
    let mut let_decls = HashMap::new();

    let_decls
      .insert("print".to_string(), Scheme::new(vec!["a".to_string()], TypeKind::print_string()));
    let_decls.insert("exit".to_string(), Scheme::new(vec!["a".to_string()], TypeKind::exit()));

    Self {
      variables: HashMap::new(),
      type_variables: HashMap::new(),
      let_decls,
      enum_decls: HashMap::new(),
      variant_to_enum: HashMap::new(),
      level: 0,
      counter: 0,
      reporter,
    }
  }

  pub fn instantiate(&mut self, scheme: Scheme) -> Type {
    let substitutions = scheme.binds.iter().map(|_| self.new_hole()).collect::<Vec<_>>();

    scheme.t.instantiate(&substitutions)
  }

  pub fn new_name(&mut self) -> String {
    self.counter += 1;
    format!("t_{}", self.counter)
  }

  pub fn new_hole(&mut self) -> Type {
    let level = self.level;
    Type::new(TypeKind::Hole { hole: Hole::new(self.new_name(), level) })
  }

  pub fn new_hole_named(&mut self, name: String) -> Type {
    let level = self.level;
    Type::new(TypeKind::Hole { hole: Hole::new(name, level) })
  }

  pub fn enter_level(&mut self) {
    self.level += 1;
  }

  pub fn leave_level(&mut self) {
    self.level -= 1;
  }

  pub fn generalize(&mut self, t: Type) -> Scheme {
    let mut counter = 0;
    let level = self.level;

    fn gen(t: Type, level: usize, counter: &mut usize) {
      match &*t {
        TypeKind::Hole { hole } => match hole.get() {
          HoleKind::Bound { t } => gen(t, level, counter),
          HoleKind::Unbound { name: _, level: hole_level } => {
            if hole_level > level {
              let current_level = *counter;
              *counter += 1;
              hole.fill(Type::new(TypeKind::Generalized { id: current_level }));
            }
          }
        },

        TypeKind::Arrow { t1, t2 } => {
          gen(t1.clone(), level, counter);
          gen(t2.clone(), level, counter);
        }

        _ => {}
      }
    }

    gen(t.clone(), level, &mut counter);

    let binds = (0..counter).map(|_| self.new_name()).collect::<Vec<_>>();
    Scheme { binds, t }
  }

  pub fn fetch(&self, name: &String) -> Option<&Scheme> {
    self.variables.get(name).or(self.let_decls.get(name))
  }
}

impl Infer for Program {
  type Out = elab::Program;

  fn infer(self, mut env: Env) -> (Self::Out, Type) {
    let mut declarations = Vec::with_capacity(self.declarations.len());

    for decl in self.declarations {
      let ((new_env, elab_decl), _) = decl.infer(env);
      env = new_env;
      declarations.push(elab_decl);
    }

    (elab::Program { file_name: self.file_name, declarations }, TypeKind::boolean())
  }
}
