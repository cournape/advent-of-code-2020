use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

extern crate clap;
use clap::{Arg, App};

const N_ROWS: u32 = 128;
const N_COLS: u32 = 8;

// seat -1 and seat +1 exist, so the row we want has exactly one hole
const INCOMPLETE_ROW: usize = N_COLS as usize - 1;

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
			let seat_id = solve_secondary_problem(&filename);
			println!("my seat id is {}", seat_id);
		}
	}
}

fn compute_row(row_string: &String) -> u32 {
	let mut min: u32 = 0;
	let mut max: u32 = N_ROWS - 1;

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
	let mut max: u32 = N_COLS - 1;

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

fn compute_seat_coordinates(line: &String) -> (u32, u32) {
	if line.len() != 10 {
		panic!("Invalid entry: {}", line);
	}

	let row = compute_row(&line[..7].to_string());
	let col = compute_col(&line[7..].to_string());

	(row, col)
}

fn compute_seat_id(line: &String) -> u32 {
	let (row, col) = compute_seat_coordinates(line);
	return row * N_COLS + col;
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

fn solve_secondary_problem(filename: &String) -> u32 {
	let file = File::open(filename).expect("failed to open");
	let buf = io::BufReader::new(file);

	let mut assigned_ids = Vec::<Vec::<u32>>::new();

	for _ in 0..N_ROWS {
		assigned_ids.push(Vec::<u32>::new());
	}

	for line in buf.lines() {
		let line = line.unwrap();
		let (row, col) = compute_seat_coordinates(&line);
		assigned_ids[row as usize].push(col);
	}

	let mut all_ids = HashSet::<u32>::with_capacity(N_COLS as usize);
	for i in 0..N_COLS {
		all_ids.insert(i);
	}

	for row in 0..N_ROWS {
		let row_ids = &assigned_ids[row as usize];
		match row_ids.len() {
			INCOMPLETE_ROW => {
				for col in row_ids {
					all_ids.remove(col);
				}
				let seat_col = all_ids.iter().next().unwrap();
				return row * 8 + seat_col;
			}
			_ => (),
		}
	}

	panic!("No proper row found, bug or bad input");
}
