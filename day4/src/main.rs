use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

extern crate clap;
use clap::{Arg, App};

fn main() {
	let matches = App::new("day 4")
                          .arg(Arg::with_name("INPUT")
                               .help("Sets the input file to use")
                               .required(true)
                               .index(1))
                          .get_matches();

	let filename = matches.value_of("INPUT").unwrap().to_string();
	println!("Using input file: {}", filename);

	let n_valid = solve_main_problem(&filename);

	println!("Found {} valid entries", n_valid);
}

fn is_valid_entry(data: &String) -> bool {
	// We use a set instead of count to handle the case where a field may
	// be repeated.
	let mut fields = HashSet::new();

	for part in data.split_whitespace() {
		let pos = part.find(":").unwrap();
		let field_name = &part[..pos];
		match field_name {
			"byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" | "cid" => {
				fields.insert(field_name);
			},
			_ => panic!("Unexpected field: {}", field_name),
		};
	}

	fields.len() == 8 || (fields.len() == 7 && !fields.contains("cid"))
}

fn solve_main_problem(filename: &String) -> usize {
	let file = File::open(filename).expect("failed to open");

	let buf = io::BufReader::new(file);
	let mut chunk = Vec::new();

	let mut n_valid: usize = 0;

	let mut process_data = |data| {
		if is_valid_entry(&data) {
			n_valid += 1;
		} else {
			println!("{}", data);
		}
	};

	for line in buf.lines() {
		let line = line.unwrap();
		if line.trim().is_empty() {
			let data = chunk.join(" ");
			process_data(data);
			chunk.clear();
		} else {
			chunk.push(line);
		}
	}

	if chunk.len() > 0 {
		let data = chunk.join(" ");
		process_data(data);
	}

	n_valid
}
