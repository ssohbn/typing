use std::io::{self, Write};

fn main() {
	println!("herlo");
	let text = get_text();
	for word in text {
		println!("quick! type the word: {}", word);
		let input = process_input();
		println!("you wrote: {}", input);
		if word.eq(&input) {
			println!("you did good!");
		}
	}		
}

fn process_input() -> String {
	print!("> ");	
	io::stdout().flush().unwrap();
	let mut buffer = String::new();	
	io::stdin().read_line(&mut buffer).unwrap();
	let trimmed = buffer.trim().to_string();
	trimmed
}

fn get_text() -> Vec<&'static str> {
	vec!["dog", "frog", "god"]
}