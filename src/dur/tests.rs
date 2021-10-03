use std::time::Duration;

fn sec(n: u64) -> Duration {
	Duration::from_secs(n)
}

fn min(n: u64) -> Duration {
	Duration::from_secs(n * 60)
}

fn hour(n: u64) -> Duration {
	Duration::from_secs(n * 3600)
}

fn day(n: u64) -> Duration {
	Duration::from_secs(n * 24 * 3600)
}

#[test]
fn parse() {
	let tests = &[
		("55s", sec(55)),
		("4h2m", sec(4 * 3600 + 2 * 60)),
		("1y 5hours 10 min", day(365) + hour(5) + min(10)),
		("1w2s 3ms", day(7) + sec(2) + Duration::from_millis(3)),
		("5", sec(5)),
		("1s2s3s4s5s", sec(15)),
		(
			"4nanos 4weeks4µs",
			day(28) + Duration::from_micros(4) + Duration::from_nanos(4),
		),
	];

	for (s, expected) in tests {
		let got = super::parse(s).unwrap();
		assert_eq!(*expected, got);
	}
}

#[test]
fn parse_fail() {
	use super::ErrorKind;

	fn unknown(s: &str) -> ErrorKind {
		ErrorKind::UnknownUnit(s.to_string())
	}
	fn unexpected(expected: &'static str, got: &'static str) -> ErrorKind {
		ErrorKind::UnexpectedToken { expected, got }
	}

	let tests = &[
		("5lmao", unknown("lmao")),
		("sec", unexpected("Number", "Unit")),
		("-2s", ErrorKind::IllegalChar('-')),
		("2d 5p", unknown("p")),
		("", ErrorKind::Empty),
	];

	for (s, expected) in tests {
		let got = super::parse(s).unwrap_err();
		assert_eq!(expected, &got.kind);
	}
}
