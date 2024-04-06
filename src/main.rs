use lalrpop_util::lalrpop_mod;

use crate::checker::{infer::Infer, Env};

pub mod ast;
pub mod checker;
pub mod elab;
lalrpop_mod!(pub parser);

fn main() {
  let tl_parser = parser::ProgramParser::new();

  let program = r#"
    enum bool_enum :=
      .True,
      .False,

    let rec up_to_zero n :=
      if n > 0
        then up_to_zero (n - 1)
        else true

    let first x y := x

    let main _ :=
      first "hello"
  "#;

  match tl_parser.parse(program) {
    Ok(program) => {
      let env = Env::default();
      let (elab_prog, _) = program.infer(env);
      println!("{elab_prog}");
    }
    Err(e) => eprintln!("{e}"),
  }
}
