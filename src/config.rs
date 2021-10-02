mod internal;

pub use internal::Error;
use serde::Deserialize;

use crate::target::Target;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
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

pub struct Config {
	pub options: Options,
	pub targets: Vec<Target>,
}
