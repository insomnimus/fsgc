use std::{
	fmt,
	fs,
	io,
	time::{
		Duration,
		SystemTime,
	},
};

use remove_dir_all::remove_dir_all;

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
			Self::Io(e) => write!(f, "{}", e),
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

pub struct Target {
	glob: String,
	age_limit: Duration,
}

impl Target {
	pub fn new(glob: String, age_limit: Duration) -> Self {
		Self { glob, age_limit }
	}

	pub fn clear(&self) -> Result<(), Error> {
		let errs: Vec<_> = glob::glob(&self.glob)?
			.filter_map(|r| {
				r.map_err(Error::from)
					.and_then(|p| {
						let md = p.metadata()?;
						if !should_delete(&md, self.age_limit) {
							Ok(())
						} else if md.is_dir() {
							remove_dir_all(&p).map_err(Error::from)
						} else {
							fs::remove_file(&p).map_err(Error::from)
						}
					})
					.err()
			})
			.collect();

		if errs.is_empty() {
			Ok(())
		} else {
			Err(Error::Many(errs))
		}
	}
}

fn should_delete(md: &fs::Metadata, age_limit: Duration) -> bool {
	let limit = SystemTime::now() - age_limit;

	if let Ok(created) = md.created() {
		if created > limit {
			return false;
		}
	}

	if let Ok(modified) = md.modified() {
		if modified > limit {
			return false;
		}
	}

	if let Ok(accessed) = md.accessed() {
		if accessed > limit {
			return false;
		}
	}

	true
}
