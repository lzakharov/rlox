use std::fmt;

use crate::object::Object;
use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Object),
    Unary(Token, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary(left, op, right) => write!(f, "({} {} {})", op, left, right),
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
            Expr::Literal(obj) => write!(f, "{}", obj),
            Expr::Unary(op, expr) => write!(f, "({} {})", op, expr),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::{Token, TokenType};

    #[test]
    fn test_expr_fmt() {
        let expr = Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(TokenType::Minus, "-".to_string(), None, 1),
                Box::new(Expr::Literal(Object::Num(123.0))),
            )),
            Token::new(TokenType::Star, "*".to_string(), None, 1),
            Box::new(Expr::Grouping(Box::new(Expr::Literal(Object::Num(45.67))))),
        );

        assert_eq!("(* (- 123) (group 45.67))", format!("{}", expr));
    }
}
