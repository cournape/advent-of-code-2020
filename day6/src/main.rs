use std::collections::HashSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

extern crate clap;
use clap::{Arg, App};

fn main() {
	let matches = App::new("day 6")
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
			let n_answers = solve_problem(&filename, &count_any_answer);
			println!("{} answers", n_answers);
		},
		1 | _ => {
			let n_answers = solve_problem(&filename, &count_all_answers);
			println!("{} answers", n_answers);
		}
	}
}

// Count the number of questions answered by everybody in the group
fn count_all_answers(chunk: &Vec<String>) -> u32 {
	let mut answers = HashMap::with_capacity(26);

	for line in chunk {
		for c in line.chars() {
			*answers.entry(c).or_insert(0) += 1;
		}
	}

	let n_members = chunk.len();
	let mut n_answers = 0;

	for (key, value) in &answers {
		if value == &n_members {
			n_answers += 1;
		}
	}

	n_answers
}


// Count the number of questions answered at least once
fn count_any_answer(chunk: &Vec<String>) -> u32 {
	let mut answers = HashSet::with_capacity(26);

	for line in chunk {
		for c in line.chars() {
			answers.insert(c);
		}
	}

	answers.len() as u32
}

fn solve_problem(filename: &String, counter: &dyn Fn(&Vec<String>) -> u32) -> u32 {
	let file = File::open(filename).expect("failed to open");

	let buf = io::BufReader::new(file);
	let mut chunk = Vec::new();
	let mut n_answers = 0;

	for line in buf.lines() {
		let line = line.unwrap();
		if line.trim().is_empty() {
			n_answers += count_all_answers(&chunk);
			chunk.clear();
		} else {
			chunk.push(line);
		}
	}

	if chunk.len() > 0 {
		n_answers += count_all_answers(&chunk);
	}

	n_answers
}
