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
			Ok(_)    => (),
			Err(err) => panic!("{arg}: {err}"),
		}
	}
}

fn cat(path: &String) -> Result<(), io::Error> {
	let mut buf :String = String::new();
	File::open(path)?.read_to_string(&mut buf)?;
	print!("{buf}");
	Ok(())
}
