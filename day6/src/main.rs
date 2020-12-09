use std::collections::HashSet;
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
			let n_answers = solve_problem(&filename);
			println!("{} answers", n_answers);
		},
		1 | _ => {
			panic!("Not implemented");
		}
	}
}

fn process_group(chunk: &Vec<String>) -> u32 {
	let mut answers = HashSet::with_capacity(26);

	for line in chunk {
		for c in line.chars() {
			answers.insert(c);
		}
		// println!("\t{}", line);
	}

	answers.len() as u32
}

fn solve_problem(filename: &String) -> u32 {
	let file = File::open(filename).expect("failed to open");

	let buf = io::BufReader::new(file);
	let mut chunk = Vec::new();
	let mut n_answers = 0;

	for line in buf.lines() {
		let line = line.unwrap();
		if line.trim().is_empty() {
			n_answers += process_group(&chunk);
			chunk.clear();
		} else {
			chunk.push(line);
		}
	}

	if chunk.len() > 0 {
		n_answers += process_group(&chunk);
	}

	n_answers
}
