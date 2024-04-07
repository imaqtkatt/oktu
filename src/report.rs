use core::fmt;
use std::sync::mpsc;

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
}

impl Reporter {
  pub fn new() -> (Self, mpsc::Receiver<Box<dyn Diagnostic>>) {
    let (sender, recv) = mpsc::channel();

    (Self { sender }, recv)
  }

  pub fn report(&self, diag: impl Diagnostic + 'static) {
    self.sender.send(Box::new(diag)).unwrap()
  }

  pub fn to_stdout(recv: mpsc::Receiver<Box<dyn Diagnostic>>) {
    const IDENT_SIZE: usize = 2;
    for diagnostic in recv.try_iter() {
      println!("[{}]: {}", diagnostic.severity(), diagnostic.message());
      for extra in diagnostic.extra() {
        println!("{:IDENT_SIZE$}{}", "", extra);
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
