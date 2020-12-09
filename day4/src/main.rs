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

	parse_data(&filename);
}

fn parse_data(filename: &String) -> () {
	let file = File::open(filename).expect("failed to open");

	let buf = io::BufReader::new(file);
	let mut chunk = Vec::new();

	for line in buf.lines() {
		let line = line.unwrap();
		if line.trim().is_empty() {
			println!("{}", chunk.join(" "));
			chunk.clear();
		} else {
			chunk.push(line);
		}
	}
}
