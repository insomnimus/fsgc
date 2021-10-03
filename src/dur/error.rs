use std::fmt;

type Pos = (usize, usize);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
	IllegalChar(char),
	Empty,
	UnexpectedToken {
		expected: &'static str,
		got: &'static str,
	},
	UnknownUnit(String),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Error {
	pub kind: ErrorKind,
	pub line: usize,
	pub col: usize,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}:{} | ", self.line, self.col)?;
		match &self.kind {
			ErrorKind::UnknownUnit(s) => write!(f, "unknown unit `{}`", &s),
			ErrorKind::IllegalChar(c) => write!(f, "illegal char '{}'", c),
			ErrorKind::Empty => write!(f, "empty input"),
			ErrorKind::UnexpectedToken { expected, got } => {
				write!(f, "unexpected token (expected {}, got {})", expected, got)
			}
		}
	}
}
impl Error {
	pub(crate) fn unknown_unit(s: String, (line, col): Pos) -> Self {
		Self {
			kind: ErrorKind::UnknownUnit(s),
			line,
			col,
		}
	}

	pub(crate) fn unexpected_token(
		expected: &'static str,
		got: &'static str,
		(line, col): Pos,
	) -> Self {
		Self {
			kind: ErrorKind::UnexpectedToken { expected, got },
			line,
			col,
		}
	}

	pub(crate) fn illegal(c: char, (line, col): Pos) -> Self {
		Self {
			kind: ErrorKind::IllegalChar(c),
			col,
			line,
		}
	}

	pub(crate) fn empty() -> Self {
		Self {
			kind: ErrorKind::Empty,
			col: 0,
			line: 0,
		}
	}
}

impl std::error::Error for Error {}
