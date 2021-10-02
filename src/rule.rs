use std::{
	convert::TryFrom,
	fs,
	time::{
		Duration,
		SystemTime,
	},
};

use anyhow::{
	anyhow,
	bail,
	ensure,
	Error,
};
use serde::Deserialize;
#[derive(Deserialize)]
#[serde(untagged)]
pub enum TomlRule {
	Simple(String),
	Detailed(DetailedTomlRule),
}

#[derive(Deserialize)]
pub struct DetailedTomlRule {
	age: String,
	#[serde(default = "return_true")]
	modified: bool,
	#[serde(default = "return_true")]
	accessed: bool,
	#[serde(default)]
	created: bool,
}

impl From<TomlRule> for DetailedTomlRule {
	fn from(r: TomlRule) -> Self {
		match r {
			TomlRule::Simple(age) => Self {
				age,
				created: true,
				modified: true,
				accessed: false,
			},
			TomlRule::Detailed(d) => d,
		}
	}
}

pub struct Rule {
	pub age: Duration,
	pub modified: bool,
	pub accessed: bool,
	pub created: bool,
}

impl Rule {
	pub fn should_delete(&self, md: &fs::Metadata) -> bool {
		if !(self.created || self.modified || self.accessed) {
			return false;
		}
		let limit = SystemTime::now() - self.age;

		let created = md.created();
		let modified = md.modified();
		let accessed = md.accessed();

		// It's better to be safe and not delete anything we can't check.
		if created.is_err() && modified.is_err() && accessed.is_err() {
			return false;
		}

		if let Ok(t) = created {
			if self.created && t > limit {
				return false;
			}
		}

		if let Ok(t) = modified {
			if self.modified && t > limit {
				return false;
			}
		}

		if let Ok(t) = accessed {
			if self.accessed && t > limit {
				return false;
			}
		}

		true
	}
}

fn parse_duration(s: String) -> Result<Duration, Error> {
	ensure! {
		s.len() > 1,
		"invalid time format: {:?}",
		s,
	};

	let n: u64 = match s.chars().last().unwrap() {
		's' | 'S' => 1,
		'm' | 'M' => 60,
		'h' | 'H' => 3600,
		'd' | 'D' => 24 * 3600,
		'w' | 'W' => 24 * 7 * 3600,
		invalid => bail!(
			"invalid time suffix '{}' in {:?}: valid suffixes are [s, m, h, d, w]",
			invalid,
			s
		),
	};

	s[..(s.len() - 1)]
		.parse::<u64>()
		.map(|x| Duration::from_secs(n * x))
		.map_err(|_| {
			anyhow!(
				"invalid duration value: {}: value must be a non-negative integer",
				&s[..(s.len() - 1)]
			)
		})
}

impl TryFrom<TomlRule> for Rule {
	type Error = Error;

	fn try_from(r: TomlRule) -> Result<Self, Self::Error> {
		let r = DetailedTomlRule::from(r);
		let created = r.created;
		let modified = r.modified;
		let accessed = r.accessed;
		parse_duration(r.age).map(|age| Self {
			age,
			created,
			modified,
			accessed,
		})
	}
}

fn return_true() -> bool {
	true
}
