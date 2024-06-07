use std::{io::Read, path::Path};

use lalrpop_util::lalrpop_mod;
use report::Reporter;

use crate::checker::{infer::Infer, Env};

pub mod ast;
pub mod checker;
pub mod elab;
pub mod report;
lalrpop_mod!(pub parser);

fn main() {
  if let Err(e) = run() {
    eprintln!("Error: {e}");
  }
}

fn run() -> std::io::Result<()> {
  let argv = std::env::args().collect::<Vec<_>>();

  let program_parser = parser::ProgramParser::new();
  let (reporter, recv) = Reporter::new();

  let path = Path::new(&argv[1]);
  let mut file = std::fs::File::open(path)?;
  let mut buf = String::new();
  file.read_to_string(&mut buf)?;

  let mut elab_program = elab::Program::empty();

  match program_parser.parse(&buf) {
    Ok(mut program) => {
      program.file_name = path.to_str().map(Box::from);
      let env = Env::new(reporter);
      let (program, _) = program.infer(env);
      elab_program = program;
    }
    Err(e) => eprintln!("{e}"),
  }

  Reporter::to_stdout(recv, file);

  let book = elab_program.to_bend().map_err(std::io::Error::other)?;
  println!("{}", book.display_pretty());

  Ok(())
}
