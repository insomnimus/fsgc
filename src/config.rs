use std::{
	collections::HashMap,
	fmt,
	fs,
	io,
	time::Duration,
};

use serde::Deserialize;

use crate::target::Target;

#[derive(Debug)]
pub enum Error {
	TimeSuffix(String, char),
	TimeAmount(String),
	Io(io::Error),
	Toml(toml::de::Error),
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
	fn from(e: io::Error) -> Self {
		Self::Io(e)
	}
}

impl From<toml::de::Error> for Error {
	fn from(e: toml::de::Error) -> Self {
		Self::Toml(e)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Io(e) => write!(f, "{}", e),
			Self::TimeAmount(s) => write!(f, "time amount is invalid: {:?}", s),
			Self::TimeSuffix(s, c) => write!(
				f,
				"time suffix is invalid ({:?}): {:?} is not recognized as a time unit specifier",
				s, c
			),
			Self::Toml(e) => write!(f, "{}", e),
		}
	}
}

#[derive(Deserialize)]
struct InternalConfig {
	options: Options,
	paths: HashMap<String, String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Options {
	pub log_file: Option<String>,
	pub header: Option<String>,
}

fn parse_duration(s: String) -> Result<Duration, Error> {
	if s.len() < 2 {
		return Err(Error::TimeAmount(s));
	}

	let n: u64 = match s.chars().last().unwrap() {
		's' | 'S' => 1,
		'm' | 'M' => 60,
		'h' | 'H' => 3600,
		'd' | 'D' => 24 * 3600,
		'w' | 'W' => 24 * 7 * 3600,
		unknown => return Err(Error::TimeSuffix(s, unknown)),
	};

	s[..(s.len() - 1)]
		.parse::<u64>()
		.map(|x| Duration::from_secs(n * x))
		.map_err(|_| Error::TimeAmount(s))
}

pub struct Config {
	pub log_file: Option<String>,
	pub header: String,
	pub targets: Vec<Target>,
}

impl Config {
	pub fn read_from(p: &str) -> Result<Self, Error> {
		let data = fs::read_to_string(p)?;
		let InternalConfig {
			options: Options { log_file, header },
			paths,
		} = toml::from_str(&data)?;

		let header = header.unwrap_or_else(|| String::from("---[%x %X]---"));
		paths
			.into_iter()
			.map(|(pat, dur)| -> Result<Target, Error> {
				let age_limit = parse_duration(dur)?;
				Ok(Target::new(pat, age_limit))
			})
			.collect::<Result<Vec<_>, _>>()
			.map(|targets| Self {
				log_file,
				header,
				targets,
			})
	}
}
