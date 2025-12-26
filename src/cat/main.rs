use std::env;
use std::io::{self, Read};
use std::fs::File;

fn main() {
	// why not
	let mut args: Vec<String> = env::args().skip(1).collect();

	// open stdin by default. assumes a fairly reasonable, modern unix.
	if args.len() == 0 {
		args.push(String::from("/dev/stdin"))
	}

	for arg in args {
		match cat(&arg) {
			None     => (),
			Some(err) => panic!("{arg}: {err}"),
		}
	}
}

// see cat1: Result<(), io::Error> allows to use '?'
fn cat(path: &String) -> Option<io::Error> {
	let fh = File::open(path);

	let mut fh = match fh {
		Ok(fh) => fh,
		Err(e) => return Some(e),
	};

	let mut buf = String::new();

	match fh.read_to_string(&mut buf) {
		Ok(_)  => print!("{buf}"),
		Err(e) => return Some(e),
	};

	None
}
