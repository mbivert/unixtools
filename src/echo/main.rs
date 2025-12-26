use std::env;

fn main() {
	// either &...[1..] or .trim() to remove the heading, fold-induced space
	println!("{}", &env::args().skip(1).fold("".to_string(), |acc, arg| format!("{acc} {arg}"))[1..]);
}
