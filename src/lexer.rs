use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum Token {
  /// .
  Dot,
  /// ,
  Comma,
  /// @
  At,
  /// =
  Equals,
  /// ->
  Arrow,
  /// =>
  FatArrow,
  /// let
  Let,
  /// in
  In,
  /// rec
  Rec,
  /// enum
  Enum,
  /// if
  If,
  /// then
  Then,
  /// else
  Else,
  /// match
  Match,
  /// with
  With,
  /// end
  End,
  /// :=
  Def,
  /// (a..z|_)*
  Ident {
    value: String,
    is_upper: bool,
  },
  /// . (a..z|_)*
  Variant {
    value: String,
  },
  /// fun
  Fun,
  /// true
  True,
  /// false
  False,
  /// "..."
  String {
    value: String,
  },
  /// i32 number
  Number {
    value: i32,
  },
  /// End of file
  Eof,
}

#[derive(Debug)]
pub struct Lexer<'a> {
  pub src: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
  pub fn new(src: &'a str) -> Self {
    Self {
      src: src.chars().peekable(),
    }
  }

  pub fn consume(&mut self) -> Option<char> {
    self.src.next()
  }

  pub fn just(&mut self, token: Token) -> Token {
    self.consume();
    token
  }

  pub fn take_while(&mut self, predicate: fn(char) -> bool) -> String {
    let mut value = String::new();

    while let Some(c) = self.src.peek() {
      if predicate(*c) {
        value.push(self.src.next().unwrap());
      } else {
        break;
      }
    }

    value
  }

  pub fn identifier(&mut self) -> Token {
    let identifier = self.take_while(|c| c.is_ascii_alphabetic());
    match identifier.as_ref() {
      "let" => Token::Let,
      "rec" => Token::Rec,
      "in" => Token::In,
      "enum" => Token::Enum,
      "match" => Token::Match,
      "with" => Token::With,
      "end" => Token::End,
      "if" => Token::If,
      "then" => Token::Then,
      "else" => Token::Else,
      "true" => Token::True,
      "false" => Token::False,
      _ => {
        let is_upper = identifier.chars().next().is_some_and(|c| c.is_uppercase());
        Token::Ident {
          value: identifier,
          is_upper,
        }
      }
    }
  }

  pub fn number(&mut self) -> Token {
    let number = self.take_while(|c| c.is_numeric());
    Token::Number {
      value: number.parse().unwrap(),
    }
  }

  pub fn next_token(&mut self) -> Token {
    if let Some(c) = self.src.peek() {
      match c {
        ' ' | '\t' | '\n' | '\r' => {
          self.consume();
          self.next_token()
        }
        '.' => {
          self.consume();
          let variant = self.take_while(|c| c.is_ascii_alphabetic());
          Token::Variant { value: variant }
        }
        ',' => self.just(Token::Comma),
        '@' => self.just(Token::At),
        ':' => {
          self.consume();
          match self.src.peek() {
            Some('=') => self.just(Token::Def),
            _ => panic!("Unexpected ':'."),
          }
        }
        '-' => {
          // TODO
          self.consume();
          match self.src.peek() {
            Some('>') => self.just(Token::Arrow),
            _ => panic!(),
          }
        }
        '=' => {
          self.consume();
          match self.src.peek() {
            Some('>') => self.just(Token::FatArrow),
            _ => Token::Equals,
          }
        }
        '"' => {
          self.consume();
          let contents = self.take_while(|c| c != '"');
          self.consume();
          Token::String { value: contents }
        }
        c if c.is_ascii_alphabetic() => self.identifier(),
        c if c.is_numeric() => self.number(),
        c => panic!("Unexpected '{c}'."),
      }
    } else {
      Token::Eof
    }
  }
}
