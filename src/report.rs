use core::fmt;
use std::{
  io::{Read, Seek, SeekFrom},
  sync::mpsc,
};

use crate::ast::Src;

#[derive(Clone)]
pub struct Reporter {
  sender: mpsc::Sender<Box<dyn Diagnostic>>,
}

pub enum Severity {
  Error,
  Warning,
  Info,
}

pub trait Diagnostic {
  fn message(&self) -> String;

  fn severity(&self) -> Severity;

  fn extra(&self) -> Vec<String>;

  fn src(&self) -> Option<Src>;
}

impl Reporter {
  pub fn new() -> (Self, mpsc::Receiver<Box<dyn Diagnostic>>) {
    let (sender, recv) = mpsc::channel();

    (Self { sender }, recv)
  }

  pub fn report(&self, diag: impl Diagnostic + 'static) {
    self.sender.send(Box::new(diag)).unwrap()
  }

  pub fn to_stdout(recv: mpsc::Receiver<Box<dyn Diagnostic>>, mut file: std::fs::File) {
    const IDENT_SIZE: usize = 2;
    for diagnostic in recv.try_iter() {
      eprintln!("[{}]: {}", diagnostic.severity(), diagnostic.message());
      if let Some(Src(pos)) = diagnostic.src() {
        file.seek(SeekFrom::Start(pos.start as u64)).unwrap();
        let len = pos.end - pos.start;
        let mut buf = vec![0; len];
        file.read_exact(&mut buf).unwrap();
        let buf = String::from_utf8(buf).unwrap();
        eprintln!("\x1b[31m{:IDENT_SIZE$}{buf}\x1b[0m", "");
        eprintln!("\x1b[31m{:IDENT_SIZE$}{:^<len$}\x1b[0m", "", "");
      }
      for extra in diagnostic.extra() {
        eprintln!("{:IDENT_SIZE$}{}", "", extra);
      }
    }
  }
}

impl fmt::Display for Severity {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Severity::Error => write!(f, "Error"),
      Severity::Warning => write!(f, "Warn"),
      Severity::Info => write!(f, "Info"),
    }
  }
}
