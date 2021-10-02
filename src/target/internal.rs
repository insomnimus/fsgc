use std::{
	collections::vec_deque::{
		self,
		VecDeque,
	},
	fmt,
	io,
};

#[derive(Debug)]
pub enum Error {
	Io(io::Error),
	Glob(glob::GlobError),
	Pattern(glob::PatternError),
	Many(Vec<Self>),
}

impl std::error::Error for Error {}

impl From<glob::PatternError> for Error {
	fn from(e: glob::PatternError) -> Self {
		Self::Pattern(e)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Io(e) => write!(f, "{:?}", e),
			Self::Glob(e) => write!(f, "{}", e),
			Self::Pattern(e) => write!(f, "{}", e),
			Self::Many(errs) => {
				for e in errs {
					writeln!(f, "-  {}", e)?;
				}
				Ok(())
			}
		}
	}
}

impl From<io::Error> for Error {
	fn from(e: io::Error) -> Self {
		Self::Io(e)
	}
}

impl From<glob::GlobError> for Error {
	fn from(e: glob::GlobError) -> Self {
		Self::Glob(e)
	}
}

impl Error {
	fn is_many(&self) -> bool {
		matches!(self, Self::Many(_))
	}
}

impl IntoIterator for Error {
	type IntoIter = vec_deque::IntoIter<Self::Item>;
	type Item = Self;

	fn into_iter(self) -> Self::IntoIter {
		let mut buf = VecDeque::new();
		match self {
			Self::Many(errs) => {
				for e in errs {
					if e.is_many() {
						buf.extend(e);
					} else {
						buf.push_back(e);
					}
				}
			}
			other => buf.push_back(other),
		};

		buf.into_iter()
	}
}
