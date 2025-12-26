use std::env;

fn main() {
	let args = env::args().skip(1);
	let n = args.len();

	for (i, arg) in args.enumerate() {
		print!("{arg}");
		if i < n-1 {
			print!(" ");
		}
	}

	print!("\n");
}
