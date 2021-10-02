use std::{
	error::Error,
	fs::OpenOptions,
	io::{
		self,
		Write,
	},
};

use chrono::Local;
use config::Config;

mod app;
mod config;
mod target;

fn run() -> Result<(), Box<dyn Error>> {
	let m = app::new().get_matches();
	let Config {
		options: config,
		targets,
	} = Config::read_from(m.value_of("config").unwrap())?;

	let stderr = io::stderr();

	let mut log_out: Box<dyn Write> = match &config.log_file {
		None => Box::new(stderr.lock()),
		Some(p) => OpenOptions::new()
			.write(true)
			.append(!config.overwrite_logs)
			.create(true)
			.open(&p)
			.map(Box::new)?,
	};

	let now = Local::now();
	writeln!(&mut log_out, "{}", now.format(&config.header))?;

	for target in &targets {
		if let Err(e) = target.clear() {
			for e in e.into_iter() {
				writeln!(&mut log_out, "{}{}", &config.error_prefix, e)?;
			}
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
