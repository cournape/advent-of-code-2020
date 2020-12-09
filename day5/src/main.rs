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
                          .arg(Arg::with_name("part-two")
			       .short("2")
			       .long("part-two")
                               .help("If set, solves part 2"))
                          .get_matches();

	let filename = matches.value_of("INPUT").unwrap().to_string();
	println!("Using input file: {}", filename);

	match matches.occurrences_of("part-two") {
		0 => {
			let max_seat_id = solve_main_problem(&filename);
			println!("max seat id is {}", max_seat_id);
		},
		1 | _ => {
			// let n_valid = solve_problem(&filename, &has_valid_values);
			// println!("Found {} valid entries", n_valid);
		}
	}
}

fn compute_row(row_string: &String) -> u32 {
	let mut min: u32 = 0;
	let mut max: u32 = 127;

	for c in row_string.chars() {
		match c {
			'B' => {
				let mut range = max - min + 1;
				range /= 2;
				min += range;
			}
			'F' => {
				let mut range = max - min + 1;
				range /= 2;
				max -= range;
			}
			_ => panic!("Invalid entry: {}", row_string),
		}
	}
	if min != max {
		panic!("Bug, min != max");
	}

	min
}

fn compute_col(row_string: &String) -> u32 {
	let mut min: u32 = 0;
	let mut max: u32 = 7;

	for c in row_string.chars() {
		match c {
			'R' => {
				let mut range = max - min + 1;
				range /= 2;
				min += range;
			}
			'L' => {
				let mut range = max - min + 1;
				range /= 2;
				max -= range;
			}
			_ => panic!("Invalid entry: {}", row_string),
		}
	}
	if min != max {
		panic!("Bug, min != max");
	}

	min
}

fn compute_seat_id(line: &String) -> u32 {
	if line.len() != 10 {
		panic!("Invalid entry: {}", line);
	}

	let row = compute_row(&line[..7].to_string());
	let col = compute_col(&line[7..].to_string());

	return row * 8 + col;
}

fn solve_main_problem(filename: &String) -> u32 {
	let file = File::open(filename).expect("failed to open");
	let buf = io::BufReader::new(file);

	let mut max_seat_id = 0;

	for line in buf.lines() {
		let line = line.unwrap();
		let seat_id = compute_seat_id(&line);
		if seat_id > max_seat_id {
			max_seat_id = seat_id;
		}
	}

	max_seat_id
}
