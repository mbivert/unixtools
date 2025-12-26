use std::env;

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();
	let mut out = String::new();

	for arg in &args {
		out.push_str(format!("{arg} ").as_str());
	}

	// drop final space
	out.pop();

	println!("{out}");
}
