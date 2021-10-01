use clap::{
	crate_version,
	App,
	AppSettings,
	Arg,
};

#[cfg(windows)]
const CONFIG_FILE: &str = r"C:\programdata\.fsgc.toml";
#[cfg(not(windows))]
const CONFIG_FILE: &str = "/etc/fsgc/fsgc.toml";

pub fn new() -> App<'static> {
	App::new("fsgc")
		.version(crate_version!())
		.about("Filesystem Garbage Collector, clears old files according to a configuration.")
		.setting(AppSettings::UnifiedHelpMessage)
		.arg(
			Arg::new("config")
				.default_value(CONFIG_FILE)
				.env("FSGC_CONFIG_PATH")
				.about("The path of the config file in TOML format."),
		)
}
