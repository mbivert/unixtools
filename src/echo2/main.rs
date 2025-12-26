use std::collections::VecDeque;
use std::env;

fn main() {
	// Vec has no pop_front() :shrug:
	let mut args: VecDeque<String> = env::args().collect();
	args.pop_front();
	let n = args.len();

	let mut i = 0;
	for arg in &args {
		print!("{arg}");
		if i < n-1 {
			print!(" ");
		}
		i += 1;
	}

	print!("\n");
}
