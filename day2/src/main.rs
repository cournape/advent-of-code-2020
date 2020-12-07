use std::fs::File;
use std::io::{self, BufRead};

extern crate clap;
use clap::{Arg, App};

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

	fn is_valid_secondary(&self) -> bool {
		let first: char = self.password.as_bytes()[self.min - 1] as char;
		let second: char = self.password.as_bytes()[self.max - 1] as char;
		return (first == self.character && second != self.character)
		 || (first != self.character && second == self.character);
	}
}

fn solve_main_problem(filename: &String) -> Result<usize, &'static str> {
	let file = File::open(filename).expect("failed to open");

	let buf = io::BufReader::new(file);

	let mut n_valid_entries = 0;

	for line in buf.lines() {
		let line = line.unwrap();
		let entry = parse_line(&line).unwrap();
		if entry.is_valid() {
			n_valid_entries += 1
		}
	}

	Ok(n_valid_entries)
}

fn solve_second_problem(filename: &String) -> Result<usize, &'static str> {
	let file = File::open(filename).expect("failed to open");

	let buf = io::BufReader::new(file);

	let mut n_valid_entries = 0;

	for line in buf.lines() {
		let line = line.unwrap();
		let entry = parse_line(&line).unwrap();
		if entry.is_valid_secondary() {
			n_valid_entries += 1
		}
	}

	Ok(n_valid_entries)
}

fn main() {
	let matches = App::new("day 2")
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("part-two")
			       .short("2")
			       .long("part-two")
                               .help("If set, solves part 2"))
                          .get_matches();


	let filename = matches.value_of("INPUT").unwrap().to_string();
	println!("Using input file: {}", filename);

	match matches.occurrences_of("part-two") {
		0 => {
			let n = solve_main_problem(&filename).unwrap();
			println!("Found {} valid entries", n);
		},
		1 | _ => {
			let n = solve_second_problem(&filename).unwrap();
			println!("Found {} valid entries", n);
		}
	}
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
