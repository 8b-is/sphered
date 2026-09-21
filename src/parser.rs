use crate::ast::{Expr, Step};
use crate::lexer::Token;

pub fn parse(tokens: &[Token]) -> Result<Expr, String> {
    let mut p = Parser { tokens, pos: 0 };
    let e = p.parse_expr()?;
    if p.pos != tokens.len() {
        return Err(format!("trailing tokens from position {}", p.pos));
    }
    Ok(e)
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        let t = self.tokens.get(self.pos).cloned();
        if t.is_some() {
            self.pos += 1;
        }
        t
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Some(Token::LParen) => {
                self.next();
                let mut exprs = Vec::new();
                while !matches!(self.peek(), Some(Token::RParen) | None) {
                    exprs.push(self.parse_expr()?);
                }
                match self.next() {
                    Some(Token::RParen) => Ok(Expr::Sphere(exprs)),
                    _ => Err("unclosed sphere".to_string()),
                }
            }
            Some(Token::LAngle) => self.parse_txn(),
            Some(Token::Atom(_)) => match self.next() {
                Some(Token::Atom(s)) => Ok(Expr::Atom(s)),
                _ => unreachable!(),
            },
            Some(t) => Err(format!("unexpected token {t:?} in expression")),
            None => Err("unexpected end of input".to_string()),
        }
    }

    fn parse_txn(&mut self) -> Result<Expr, String> {
        self.next(); // consume '<'
        match self.next() {
            Some(Token::LParen) => {}
            _ => return Err("a transaction opens with <(".to_string()),
        }
        let mut steps = Vec::new();
        loop {
            match self.peek() {
                Some(Token::RParen) => {
                    self.next();
                    break;
                }
                Some(Token::Question) => {
                    self.next();
                    steps.push(Step::Admit(self.parse_expr()?));
                }
                Some(Token::Pipe) => {
                    self.next();
                    steps.push(Step::Bind(self.parse_expr()?));
                }
                Some(Token::Tilde) => {
                    self.next();
                    steps.push(Step::Transform(self.parse_expr()?));
                }
                Some(Token::Bang) => {
                    self.next();
                    steps.push(Step::Verify(self.parse_expr()?));
                }
                Some(Token::At) => {
                    self.next();
                    steps.push(Step::Witness(self.parse_expr()?));
                }
                Some(Token::Hash) => {
                    self.next();
                    steps.push(Step::Remember(self.parse_expr()?));
                }
                Some(Token::RAngle) => {
                    self.next();
                    steps.push(Step::Commit(self.parse_expr()?));
                }
                None => return Err("unclosed transaction".to_string()),
                _ => steps.push(Step::Plain(self.parse_expr()?)),
            }
        }
        match self.next() {
            Some(Token::RAngle) => Ok(Expr::Txn(steps)),
            _ => Err("a transaction closes with >".to_string()),
        }
    }
}
