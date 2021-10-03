use std::{
	fs,
	time::{
		self,
		SystemTime,
	},
};

use serde::{
	de::Deserializer,
	Deserialize,
};

use crate::dur::Duration;

#[derive(Deserialize)]
#[serde(untagged)]
enum TomlRule {
	Simple(Duration),
	Detailed(DetailedRule),
}

#[derive(Deserialize)]
struct DetailedRule {
	age: Duration,
	#[serde(default = "return_true")]
	modified: bool,
	#[serde(default = "return_true")]
	accessed: bool,
	#[serde(default)]
	created: bool,
}

impl From<TomlRule> for Rule {
	fn from(r: TomlRule) -> Self {
		match r {
			TomlRule::Simple(age) => Self {
				age: age.0,
				created: true,
				modified: true,
				accessed: false,
			},
			TomlRule::Detailed(d) => Self {
				age: d.age.0,
				created: d.created,
				modified: d.modified,
				accessed: d.accessed,
			},
		}
	}
}

#[derive(Copy, Clone)]
pub struct Rule {
	age: time::Duration,
	modified: bool,
	accessed: bool,
	created: bool,
}

impl<'de> serde::Deserialize<'de> for Rule {
	fn deserialize<D>(des: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		TomlRule::deserialize(des).map(Self::from)
	}
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

fn return_true() -> bool {
	true
}
