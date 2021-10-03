mod error;
mod parser;
#[cfg(test)]
mod tests;

use std::{
	fmt,
	time,
};

pub use error::*;
pub use parser::parse;
use serde::de::{
	self,
	Deserialize,
	Deserializer,
	Visitor,
};

#[derive(Copy, Clone)]
pub struct Duration(pub(crate) time::Duration);

struct DurationVisitor;

impl<'de> Visitor<'de> for DurationVisitor {
	type Value = Duration;

	fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "a valid duration in the form `<Number><Unit>`")
	}

	fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		parser::parse(s)
			.map(Duration)
			.map_err(|e| E::custom(e.to_string()))
	}
}

impl<'de> Deserialize<'de> for Duration {
	fn deserialize<D>(des: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		des.deserialize_str(DurationVisitor)
	}
}
