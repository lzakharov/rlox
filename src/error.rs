#[derive(Debug, Clone)]
pub struct Error {
    line: usize,
    message: String,
}

impl Error {
    pub fn new(line: usize, message: String) -> Error {
        Error {
            line: line,
            message: message,
        }
    }

    pub fn report(&self, loc: &str) {
        eprintln!("[line {}] Error + {}: {}", self.line, loc, self.message)
    }
}
