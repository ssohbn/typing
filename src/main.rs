use std::io::{self, Write};

fn main() {
	println!("herlo");
	
	loop {
		let mut buffer = String::new();
		let input = process_input();
		println!("you wrote: {}", input);
		
	}
}

fn process_input() -> &str {
	print!("> ");	
	io::stdout().flush().unwrap();
	let mut buffer = String::new();	
	io::stdin().read_line(&mut buffer).unwrap();
	let trimmed = &(buffer.trim());
	trimmed
}