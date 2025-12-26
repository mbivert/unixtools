use std::env;
use std::io::{self, Read, Write};
use std::fs::File;

// way too small for practical uses. good to estimate whether
// what follows work.
const BUFSIZ: usize = 10;

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

// C-like buffered reading.
fn cat(path: &String) -> Result<(), io::Error> {
	let mut fh = File::open(path)?;

	let mut xs: [u8; BUFSIZ] = [0; BUFSIZ];

	loop {
		let n = fh.read(&mut xs[..])?;
		if n == 0 {
			break;
		}
		// if reading less than BUFSIZ, we'll get garbage from
		// the previous read() if we don't grab at most n bytes
		// from xs.
		io::stdout().write(&xs[..n])?;
	}

	Ok(())
}
