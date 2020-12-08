use std::fs::File;
use std::io::{self, BufRead};

use array2d::Array2D;

extern crate clap;
use clap::{Arg, App};

#[derive(Copy, Clone)]
enum LocationKind{
	Tree,
	OpenSquare,
}
	
fn main() {
	let matches = App::new("day 3")
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
			let n_trees = solve_main_problem(&filename).unwrap();
			println!("Encountered {} trees", n_trees);
		},
		_ => {
			panic!("Not supported yet");
		}
	}
}

fn solve_main_problem(filename: &String) -> Result<usize, &'static str> {
	let file = File::open(filename).expect("failed to open");

	let buf = io::BufReader::new(file);

	let mut data = Vec::<Vec::<LocationKind>>::new();

	for line in buf.lines() {
		let line = line.unwrap();
		let mut line_data = Vec::<LocationKind>::new();
		for c in line.split("") {
			match c {
				"." => line_data.push(LocationKind::OpenSquare),
				"#" => line_data.push(LocationKind::Tree),
				"" => (),
				_ => panic!("Unexpected char '{}' ", c),
			}	
		}
		data.push(line_data);
	}

	let n_rows = data.len();
	if n_rows > 0 {
		let n_cols = data[0].len();
		for row in &data {
			if row.len() != n_cols {
				panic!("Inconsistent grid");
			}
		}
	}

	let grid = Array2D::from_rows(&data);

	let mut j = 0;
	let mut n_trees = 0;

	for i in 0..grid.num_rows() {
		let pos = grid[(i, j % grid.num_columns())];
		match pos {
			LocationKind::Tree => n_trees += 1,
			LocationKind::OpenSquare => (),
		}
		j += 3;
	}

	Ok(n_trees)
}
