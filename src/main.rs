use std::{fs, io::Write};

use ast::{Arm, Literal, Pattern};

use crate::{
  ast::{Expression, Function, TopLevel},
  checker::{infer::Infer, Env},
};

pub mod ast;
pub mod checker;
pub mod elab;

fn main() {
  let decls = vec![
    TopLevel::Enum(ast::Enum {
      name: format!("bool_enum"),
      variants: vec![format!("true"), format!("false")],
    }),
    TopLevel::Function(Function {
      name: format!("up_to_zero"),
      rec: true,
      parameters: vec![format!("n")],
      body: Expression::If {
        condition: Box::new(Expression::BinaryOp {
          op: ast::Operation::Gt,
          lhs: Box::new(Expression::Variable { name: format!("n") }),
          rhs: Box::new(Expression::Literal {
            literal: Literal::Number { value: 0 },
          }),
        }),
        then: Box::new(Expression::Application {
          function: Box::new(Expression::Variable {
            name: format!("up_to_zero"),
          }),
          argument: Box::new(Expression::Hole {
            name: format!("ata"),
          }),
        }),
        otherwise: Box::new(Expression::Literal {
          literal: Literal::Boolean { value: true },
        }),
      },
    }),
    TopLevel::Function(Function {
      name: format!("first"),
      rec: false,
      parameters: vec![format!("x"), format!("y")],
      body: Expression::Variable { name: format!("x") },
    }),
    TopLevel::Function(Function {
      name: format!("main"),
      rec: false,
      parameters: vec![],
      body: Expression::Application {
        function: Box::new(Expression::Variable {
          name: format!("first"),
        }),
        argument: Box::new(Expression::Literal {
          literal: Literal::Number { value: 42 },
        }),
      },
    }),
    TopLevel::Function(Function {
      name: format!("main2"),
      rec: false,
      parameters: vec![format!("x")],
      body: Expression::Match {
        scrutinee: Box::new(Expression::Variable { name: format!("x") }),
        arms: vec![
          Arm {
            left: Pattern::Enum {
              name: format!("true"),
            },
            right: Expression::Literal {
              literal: Literal::Number { value: 42 },
            },
          },
          Arm {
            left: Pattern::Enum {
              name: format!("false"),
            },
            right: Expression::BinaryOp {
              op: ast::Operation::Add,
              lhs: Box::new(Expression::Literal {
                literal: Literal::Number { value: 2 },
              }),
              rhs: Box::new(Expression::Literal {
                literal: Literal::Number { value: 3 },
              }),
            },
          },
        ],
      },
    }),
  ];

  let mut out_infers = fs::File::options()
    .create(true)
    .write(true)
    .truncate(true)
    .open("out.toktu")
    .unwrap();

  let mut env = Env::default();
  for decl in decls {
    let ((new_env, elab_decl), decl_type) = decl.infer(env.clone());
    env = new_env;
    println!("{elab_decl} :- {decl_type}");
    match elab_decl {
      elab::TopLevel::Function(function) => {
        _ = out_infers
          .write(format!("{} :- {}\n\n", function.name, decl_type).as_bytes())
          .unwrap()
      }
      elab::TopLevel::Enum(_) => {}
    }
  }
}
