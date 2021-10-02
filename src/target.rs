mod internal;

use std::{
	fs,
	time::{
		Duration,
		SystemTime,
	},
};

pub use internal::Error;
use remove_dir_all::remove_dir_all;

pub struct Target {
	glob: String,
	age_limit: Duration,
}

impl Target {
	pub fn new(glob: String, age_limit: Duration) -> Self {
		Self { glob, age_limit }
	}

	pub fn clear(&self) -> Result<(), Error> {
		let errs: Vec<_> = glob::glob(&self.glob)?
			.filter_map(|r| {
				r.map_err(Error::from)
					.and_then(|p| {
						let md = p.metadata()?;
						if !should_delete(&md, self.age_limit) {
							Ok(())
						} else if md.is_dir() {
							remove_dir_all(&p).map_err(Error::from)
						} else {
							fs::remove_file(&p).map_err(Error::from)
						}
					})
					.err()
			})
			.collect();

		if errs.is_empty() {
			Ok(())
		} else {
			Err(Error::Many(errs))
		}
	}
}

fn should_delete(md: &fs::Metadata, age_limit: Duration) -> bool {
	let limit = SystemTime::now() - age_limit;

	if let Ok(created) = md.created() {
		if created > limit {
			return false;
		}
	}

	if let Ok(modified) = md.modified() {
		if modified > limit {
			return false;
		}
	}

	if let Ok(accessed) = md.accessed() {
		if accessed > limit {
			return false;
		}
	}

	true
}
