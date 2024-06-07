use bend::fun as bend;

use super::{Expression, Function, Literal, Operation, TopLevel};

const OKTU_BUILTINS_PATH: &str = "src/oktu.builtins.bend";

const OKTU_CONCAT: &str = "_Oktu_/String/concat";

impl Literal {
  pub fn to_bend(self) -> Result<bend::Term, String> {
    match self {
      Literal::Number { value } => Ok(bend::Term::Num {
        val: bend::Num::I24(value),
      }),
      Literal::String { value } => Ok(bend::Term::Str {
        val: bend::STRINGS.get(value),
      }),
      Literal::Boolean { value } => Ok(bend::Term::Num {
        val: bend::Num::U24(if value { 1 } else { 0 }),
      }),
    }
  }
}

impl Expression {
  pub fn to_bend(self) -> Result<bend::Term, String> {
    match self {
      Expression::Error { message } => Err(message),
      Expression::Unit => Ok(bend::Term::Era),
      Expression::Hole { name } => Err(format!("Hole '{name}'")),
      Expression::Variable { name } => Ok(bend::Term::Var {
        nam: bend::Name::new(name),
      }),
      Expression::Fun { variable, body } => Ok(bend::Term::Lam {
        tag: bend::Tag::Auto,
        pat: bend::Pattern::Var(Some(bend::Name::new(variable))).into(),
        bod: body.to_bend()?.into(),
      }),
      Expression::Application { function, argument } => Ok(bend::Term::App {
        tag: bend::Tag::Auto,
        fun: function.to_bend()?.into(),
        arg: argument.to_bend()?.into(),
      }),
      Expression::Literal { literal } => literal.to_bend(),
      Expression::Let { bind, value, next } => Ok(bend::Term::Let {
        pat: bend::Pattern::Var(Some(bend::Name::new(bind))).into(),
        val: value.to_bend()?.into(),
        nxt: next.to_bend()?.into(),
      }),
      Expression::If {
        condition,
        then,
        otherwise,
      } => Ok(bend::Term::Swt {
        bnd: None,
        arg: condition.to_bend()?.into(),
        with_bnd: vec![],
        with_arg: vec![],
        pred: None,
        arms: vec![otherwise.to_bend()?, then.to_bend()?],
      }),
      Expression::Match { .. } => Err("Match is not implemented".to_string()),
      Expression::BinaryOp { op, lhs, rhs } => {
        let fst = lhs.to_bend()?.into();
        let snd = rhs.to_bend()?.into();
        match op {
          Operation::Add => Ok(bend::Term::Oper {
            opr: bend::Op::ADD,
            fst,
            snd,
          }),
          Operation::Sub => Ok(bend::Term::Oper {
            opr: bend::Op::SUB,
            fst,
            snd,
          }),
          Operation::Mul => Ok(bend::Term::Oper {
            opr: bend::Op::MUL,
            fst,
            snd,
          }),
          Operation::Div => Ok(bend::Term::Oper {
            opr: bend::Op::DIV,
            fst,
            snd,
          }),
          Operation::Gt => Ok(bend::Term::Oper {
            opr: bend::Op::GT,
            fst,
            snd,
          }),
          Operation::Gte => todo!(),
          Operation::Lt => Ok(bend::Term::Oper {
            opr: bend::Op::LT,
            fst,
            snd,
          }),
          Operation::Lte => todo!(),
          Operation::Eq => Ok(bend::Term::Oper {
            opr: bend::Op::EQ,
            fst,
            snd,
          }),
          Operation::Neq => Ok(bend::Term::Oper {
            opr: bend::Op::NEQ,
            fst,
            snd,
          }),
          Operation::Concat => Ok(bend::Term::call(
            bend::Term::r#ref(OKTU_CONCAT),
            [*fst, *snd],
          )),
        }
      }
      Expression::Variant { variant: _ } => todo!(),
      Expression::Tuple { elements } => Ok(bend::Term::Fan {
        fan: bend::FanKind::Tup,
        tag: bend::Tag::Auto,
        els: elements.into_iter().flat_map(Self::to_bend).collect(),
      }),
    }
  }
}

pub enum BendTopLevel {
  Definition(bend::Definition),
  Adt(bend::Adt),
}

impl TopLevel {
  pub fn to_bend(self) -> Result<BendTopLevel, String> {
    match self {
      TopLevel::Function(function) => function.to_bend().map(BendTopLevel::Definition),
      TopLevel::Enum(_) => Err("Not implemented".to_string()),
    }
  }
}

impl Function {
  pub fn to_bend(self) -> Result<bend::Definition, String> {
    let name = self.name;
    let rules = vec![bend::Rule {
      pats: vec![],
      body: self.body.to_bend()?,
    }];
    Ok(bend::Definition {
      name: bend::Name::new(name),
      rules,
      builtin: false,
    })
  }
}

impl super::Program {
  pub fn to_bend(self) -> Result<bend::Book, String> {
    let oktu_builtins_path = std::path::Path::new(OKTU_BUILTINS_PATH);
    let code = std::fs::read_to_string(oktu_builtins_path).map_err(|e| e.to_string())?;
    let mut book =
      bend::load_book::do_parse_book(&code, oktu_builtins_path, bend::Book::default())?;
    for decl in self.declarations {
      match decl.to_bend()? {
        BendTopLevel::Definition(def) => _ = book.defs.insert(def.name.clone(), def),
        BendTopLevel::Adt(_) => todo!(),
      }
    }
    Ok(book)
  }
}
