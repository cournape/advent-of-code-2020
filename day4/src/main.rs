use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

use regex::Regex;

extern crate clap;
use clap::{Arg, App};

fn main() {
	let matches = App::new("day 4")
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
			let n_valid = solve_problem(&filename, &has_valid_fields);
			println!("Found {} valid entries", n_valid);
		},
		1 | _ => {
			let n_valid = solve_problem(&filename, &has_valid_values);
			println!("Found {} valid entries", n_valid);
		}
	}

}

fn has_valid_fields(data: &String) -> bool {
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

// Will return false if the value is not a number or not in the given range
fn is_number_between(value: &String, min: u32, max: u32) -> bool {
	let r = value.parse::<u32>();
	match r {
		Ok(number) => if number < min || number > max {
			return false;
		},
		Err(_) => return false,
	};

	true
}

fn has_valid_values(data: &String) -> bool {
	if !has_valid_fields(data) {
		return false;
	}
	let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
	let pid_re = Regex::new(r"^\d{9}$").unwrap();

	for part in data.split_whitespace() {
		let pos = part.find(":").unwrap();
		let field_name = &part[..pos];
		let value = &part[pos+1..];
		match field_name {
			"byr" => {
				if value.len() != 4  || !is_number_between(&value.to_string(), 1920, 2002) {
					return false;
				}
			},
			"iyr" => {
				if value.len() != 4 {
					return false;
				}
				if value.len() != 4  || !is_number_between(&value.to_string(), 2010, 2020) {
					return false;
				}
			},
			"eyr" => {
				if value.len() != 4 {
					return false;
				}
				if value.len() != 4  || !is_number_between(&value.to_string(), 2020, 2030) {
					return false;
				}
			},
			"hgt" => {
				let unit = &value[value.len() - 2..];
				let v = &value[..value.len() - 2];
				match unit {
					"cm" => if !is_number_between(&v.to_string(), 150, 193) {
						return false;
					},
					"in" => if !is_number_between(&v.to_string(), 59, 76) {
						return false;
					},
					_ => return false,
				}
			},
			"hcl" => {
				if value.len() != 7 || !hcl_re.is_match(value) {
					return  false;
				}
			},
			"ecl" => {
				match value {
					"amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
					_ => return false,
				}
			},
			"pid" => {
				if !pid_re.is_match(value) {
					return false;
				}
			},
			"cid" =>{
			},
			_ => panic!("Unexpected field: {}", field_name),
		};
	}

	true
}

fn solve_problem(filename: &String, validator: &dyn Fn(&String) -> bool) -> usize {
	let file = File::open(filename).expect("failed to open");

	let buf = io::BufReader::new(file);
	let mut chunk = Vec::new();

	let mut n_valid: usize = 0;

	let mut process_data = |data| {
		if validator(&data) {
			n_valid += 1;
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
