use std::{io::Read, path::PathBuf};

use clap::{Parser, Subcommand};
use lalrpop_util::lalrpop_mod;
use report::Reporter;

use crate::checker::{infer::Infer, Env};

pub mod ast;
pub mod checker;
pub mod elab;
pub mod report;
lalrpop_mod!(pub parser);

#[derive(Clone, Parser)]
struct Cli {
  #[command(subcommand)]
  pub command: Cmd,
}

#[derive(Clone, Subcommand)]
pub enum Cmd {
  /// Type checks the program.
  Check { path: PathBuf },
  /// Compiles the program to Bend.
  Compile { path: PathBuf },
}

fn main() {
  if let Err(e) = run() {
    eprintln!("Error: {e}");
  }
}

fn run() -> std::io::Result<()> {
  let cli = Cli::parse();
  let (reporter, recv) = Reporter::new();

  match cli.command {
    Cmd::Check { path } => {
      let mut file = std::fs::File::open(&path)?;
      let input = read_file(&mut file)?;
      match parser::ProgramParser::new().parse(&input) {
        Ok(mut program) => {
          program.set_file_name(path.to_str().map(Box::from));
          let env = Env::new(reporter);
          _ = program.infer(env);
          Reporter::to_stdout(recv, file);
        }
        Err(e) => eprintln!("{e}"),
      };
    }
    Cmd::Compile { path } => {
      let mut file = std::fs::File::open(&path)?;
      let input = read_file(&mut file)?;
      match parser::ProgramParser::new().parse(&input) {
        Ok(mut program) => {
          program.set_file_name(path.to_str().map(Box::from));
          let env = Env::new(reporter);
          let (program, _) = program.infer(env);
          Reporter::to_stdout(recv, file);
          let output = program.to_bend().map_err(std::io::Error::other)?;
          println!("{}", output.display_pretty());
        }
        Err(e) => eprintln!("{e}"),
      };
    }
  }

  Ok(())
}

fn read_file(file: &mut std::fs::File) -> Result<String, std::io::Error> {
  let mut buf = String::new();
  file.read_to_string(&mut buf)?;
  Ok(buf)
}
