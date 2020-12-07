use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Entry {
	min: usize,
	max: usize,
	character: char,
	password: String,
}

impl Entry {
	fn is_valid(&self) -> bool {
		let n = self.password.matches(&self.character.to_string()).count();
		return (self.min <= n) && (n <= self.max);
	}
}

fn main() {
	let filename = "./input";
	let file = File::open(filename).expect("failed to open");

	let buf = io::BufReader::new(file);

	let mut n_valid_entries = 0;

	for line in buf.lines() {
		let line = line.unwrap();
		let entry = parse_line(&line).unwrap();
		if entry.is_valid() {
			n_valid_entries += 1
		}
		// println!("line '{}'", line);
		// println!("\tvalid ? {}", entry.is_valid());
	}

	println!("{} valid entries", n_valid_entries);
}

fn parse_line(line: &String) -> Result<Entry, &'static str> {
	let parts = line.split_whitespace();
	let parts: Vec<&str> = parts.collect();
	match parts.len() {
		3 => {
			let left = &parts[0];
			let pos = left.find("-").unwrap();
			let min = left[..pos].parse().unwrap();
			let max = left[pos+1..].parse().unwrap();

			let middle = &parts[1];
			if middle.len() != 2 {
				return Err("Could not parse entry");
			}
			let character = middle.chars().nth(0).unwrap();

			let entry = Entry {
				min: min,
				max: max,
				character: character,
				password: parts[2].to_string(),
			};
			Ok(entry)
		},
		_ => Err("Could not parse line"),
	}
}
