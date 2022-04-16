use std::io::{self, Write};

fn main() {
	println!("herlo");
	let word = String::from("hello");
	
	loop {
		let input = process_input();
		println!("you wrote: {}", input);
		if input == word {
			println!("you win");
			break;
		}
	}
}

fn process_input() -> String {
	print!("> ");	
	io::stdout().flush().unwrap();
	let mut buffer = String::new();	
	io::stdin().read_line(&mut buffer).unwrap();
	let trimmed = &(buffer.trim());
	trimmed.to_string()
}