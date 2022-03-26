use crate::token::{Token, TokenType};

pub enum Error {
    Error { line: usize, message: String },
    ParseError { token: Token, message: String },
}

impl Error {
    pub fn error(line: usize, message: String) -> Error {
        Error::Error {
            line: line,
            message: message,
        }
    }

    pub fn parse_error(token: Token, message: String) -> Error {
        Error::ParseError {
            token: token,
            message: message,
        }
    }

    pub fn report(&self, loc: &str) {
        match self {
            Error::Error { line, message } => {
                eprintln!("[line {}] Error + {}: {}", line, loc, message);
            }
            Error::ParseError { token, message } => {
                if token.is(TokenType::Eof) {
                    eprintln!("Error + {}: {}", loc, message);
                } else {
                    eprintln!("[line {}] Error + {}: {}", token.get_line(), loc, message);
                }
            }
        }
    }
}
