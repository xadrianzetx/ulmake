use std::fmt;
use std::fmt::Formatter;

pub enum GameStatus {
    Good,
    Bad(String),
}

impl From<&str> for GameStatus {
    fn from(message: &str) -> Self {
        GameStatus::Bad(String::from(message))
    }
}

impl fmt::Display for GameStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GameStatus::Good => write!(f, "OK"),
            GameStatus::Bad(s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_ok() {
        assert_eq!(format!("{}", GameStatus::Good), "OK");
    }

    #[test]
    fn test_format_err() {
        assert_eq!(format!("{}", GameStatus::from("foo")), "foo");
    }
}
