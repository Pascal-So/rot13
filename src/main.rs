use std::io;
use std::ascii::AsciiExt;

fn rot13_char(c : char) -> char {
	if c.is_ascii() {
		let ord = c as u8;
		if ord >= 65 && ord <= 90 { // uppercase chars
			return ((ord - 65 + 13) % 26 + 65) as char;
		}else if ord >= 97 && ord <= 122 { // lowercase chars
			return ((ord - 97 + 13) % 26 + 97) as char;
		}
	}
	c
}

fn rot13(input : &String) -> String{
	input.chars()
		.map(|c| rot13_char(c))
		.collect()
}

fn main() {
	let mut input = String::new();
	while let Ok(nr_bytes) = io::stdin().read_line(&mut input) {
		if nr_bytes == 0 {
			// reached EOF
			break;
		}
		print!("{}", rot13(&input));
		input = "".to_string();
	}
}
