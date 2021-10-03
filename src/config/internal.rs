use std::{
	collections::HashMap,
	convert::TryFrom,
	fs,
};

use anyhow::{
	Context,
	Error,
};
use serde::Deserialize;

use super::Options;
use crate::{
	rule::{
		Rule,
		TomlRule,
	},
	target::Target,
};

#[derive(Deserialize)]
struct Config {
	options: Options,
	rules: HashMap<String, Rule>,
}

impl super::Config {
	pub fn read_from(p: &str) -> Result<Self, Error> {
		let data = fs::read_to_string(p).with_context(|| format!("unable to read {}", p))?;

		let Config { options, rules } = toml::from_str(&data).context("malformed TOML file")?;

		let targets = rules
			.into_iter()
			.map(|(pat, rule)| {
				Target::new(pat, rule)
			})
			.collect::<Vec<_>>();
			
			Ok(Self{options, targets})
	}
}
