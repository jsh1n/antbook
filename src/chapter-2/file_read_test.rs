use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let f = File::open("hello.txt").unwrap();
	let mut reader = BufReader::new(f);
	cat_file(&mut reader);
}

fn cat_file(stream: &mut BufRead) {
	let mut buffer = String::new();
	loop {
		match stream.read_line(&mut buffer) {
			Ok(0) => break, // EOF
			Ok(_) => {
				print!("{}", buffer);
				buffer.clear();
			}
			Err(e) => {
				println!("{}", e);
				break;
			}
		}
	}
}
