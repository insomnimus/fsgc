use std::{
	iter::Peekable,
	str::Chars,
	time::Duration,
};

use super::Error;

type TokenResult = Result<(Token, (usize, usize)), Error>;

enum Unit {
	Nanos,
	Micros,
	Millis,
	Sec,
	Min,
	Hour,
	Day,
	Week,
	Year,
}

enum Token {
	Unit(Unit),
	Int(u64),
}

impl Token {
	fn kind(&self) -> &'static str {
		match self {
			Self::Unit(_) => "Unit",
			Self::Int(_) => "Number",
		}
	}
}

struct Parser<'a> {
	line: usize,
	col: usize,
	input: Peekable<Chars<'a>>,
}

impl<'a> Parser<'a> {
	fn new(s: &'a str) -> Self {
		Self {
			line: 0,
			col: 0,
			input: s.chars().peekable(),
		}
	}

	fn next(&mut self) -> Option<TokenResult> {
		while let Some(c) = self.input.next_if(|c| c.is_whitespace()) {
			if c == '\n' {
				self.col = 0;
				self.line += 1;
			} else if c != '\r' {
				self.col += 1;
			}
		}

		self.input.peek().copied().map(|c| {
			if c.is_digit(10) {
				self.parse_int()
			} else if c.is_alphabetic() {
				self.parse_unit()
			} else {
				Err(Error::illegal(c, (self.line, self.col)))
			}
		})
	}

	fn parse_int(&mut self) -> TokenResult {
		let mut buf = String::with_capacity(4);
		let col = self.col;
		while let Some(c) = self.input.next_if(|c| c.is_digit(10)) {
			self.col += 1;
			buf.push(c);
		}

		Ok((Token::Int(buf.parse::<u64>().unwrap()), (self.line, col)))
	}

	fn parse_unit(&mut self) -> TokenResult {
		let col = self.col;
		use Unit::*;
		let mut buf = String::with_capacity(4);
		while let Some(c) = self.input.next_if(|c| c.is_alphabetic()) {
			self.col += 1;
			buf.extend(c.to_lowercase());
		}

		Ok((
			Token::Unit(match &buf[..] {
				"ns" | "nanos" | "nanoseconds" | "nanosecond" => Nanos,
				"µs" | "micros" | "microseconds" | "microsecond" => Micros,
				"ms" | "millis" | "milliseconds" | "millisecond" => Millis,
				"s" | "sec" | "secs" | "seconds" | "second" => Sec,
				"m" | "min" | "minute" | "minutes" | "mins" => Min,
				"h" | "hr" | "hours" | "hour" => Hour,
				"d" | "day" | "days" => Day,
				"w" | "week" | "weeks" => Week,
				"y" | "yr" | "years" | "year" => Year,
				_ => return Err(Error::unknown_unit(buf, (self.line, col))),
			}),
			(self.line, col),
		))
	}
}

pub fn parse(s: &str) -> Result<Duration, Error> {
	let mut not_empty = false;
	let mut p = Parser::new(s);
	let mut t = Duration::default();

	while let Some(token) = p.next() {
		not_empty = true;
		let (token, pos) = token?;
		match token {
			Token::Unit(_) => return Err(Error::unexpected_token("Number", token.kind(), pos)),
			Token::Int(n) => {
				if let Some(next) = p.next() {
					let (next, pos) = next?;
					if let Token::Unit(unit) = next {
						t += unit.duration(n);
					} else {
						return Err(Error::unexpected_token("Unit", next.kind(), pos));
					}
				} else {
					t += Duration::from_secs(n);
					break;
				}
			}
		}
	}

	if not_empty {
		Ok(t)
	} else {
		Err(Error::empty())
	}
}

impl Unit {
	fn duration(&self, n: u64) -> Duration {
		match self {
			Self::Nanos => Duration::from_nanos(n),
			Self::Micros => Duration::from_micros(n),
			Self::Millis => Duration::from_millis(n),
			Self::Sec => Duration::from_secs(n),
			Self::Min => Duration::from_secs(n * 60),
			Self::Hour => Duration::from_secs(3600 * n),
			Self::Day => Duration::from_secs(3600 * 24 * n),
			Self::Week => Duration::from_secs(3600 * 24 * 7 * n),
			Self::Year => Duration::from_secs(3600 * 24 * 365 * n),
		}
	}
}
