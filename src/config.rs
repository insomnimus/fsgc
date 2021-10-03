use std::{
	collections::HashMap,
	fs,
};

use anyhow::{
	Context,
	Error,
};
use serde::Deserialize;

use crate::rule::Rule;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", default, deny_unknown_fields)]
pub struct Options {
	pub log_file: Option<String>,
	pub overwrite_logs: bool,
	pub header: String,
	pub error_prefix: String,
}

impl Default for Options {
	fn default() -> Self {
		Self {
			log_file: None,
			overwrite_logs: false,
			header: String::from("---[%x %X]---"),
			error_prefix: String::from("\terror: "),
		}
	}
}

#[derive(Deserialize)]
pub struct Config {
	pub options: Options,
	#[serde(rename = "rules")]
	pub targets: HashMap<String, Rule>,
}

impl Config {
	pub fn read_from(p: &str) -> Result<Self, Error> {
		let data = fs::read_to_string(p).with_context(|| format!("unable to read {}", p))?;
		toml::from_str(&data).context("malformed TOML file")
	}
}
