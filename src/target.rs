mod internal;

use std::fs;

pub use internal::Error;
use remove_dir_all::remove_dir_all;

use crate::rule::Rule;

pub struct Target<'a> {
	glob: &'a str,
	rule: Rule,
}

impl<'a> Target<'a> {
	pub fn new(glob: &'a str, rule: Rule) -> Self {
		Self { glob, rule }
	}

	pub fn clear(&self) -> Result<(), Error> {
		let errs: Vec<_> = glob::glob(self.glob)?
			.filter_map(|r| {
				r.map_err(Error::from)
					.and_then(|p| {
						let md = p.metadata()?;
						if !self.rule.should_delete(&md) {
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
