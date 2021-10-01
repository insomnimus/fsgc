#![feature(stdio_locked)]

use std::{
	error::Error,
	fs::OpenOptions,
	io::{
		self,
		Write,
	},
};

use config::Config;

mod app;
mod config;
mod target;

fn run() -> Result<(), Box<dyn Error>> {
	let m = app::new().get_matches();
	let config = Config::read_from(m.value_of("config").unwrap())?;

	let mut err_out: Box<dyn Write> = match &config.log_file {
		None => {
			let stderr = io::stderr();
			Box::new(stderr.into_locked())
		}
		Some(p) => OpenOptions::new()
			.append(true)
			.create(true)
			.open(&p)
			.map(Box::new)?,
	};

	writeln!(&mut err_out, "---NEW CYCLE---")?;
	for target in &config.targets {
		if let Err(e) = target.clear() {
			writeln!(&mut err_out, "{}", e)?;
		}
	}
	Ok(())
}

fn main() {
	if let Err(e) = run() {
		eprintln!("fatal: {}", e);
		std::process::exit(1);
	}
}
