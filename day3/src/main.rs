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
		1 | _ => {
			let n_trees = solve_aux_problem(&filename).unwrap();
			println!("Encountered {} trees", n_trees);
		}
	}
}

fn parse_grid(filename: &String) -> Result<Array2D::<LocationKind>, &'static str> {
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

	return Ok(Array2D::from_rows(&data));
}

fn solve_for_slope(grid: &Array2D<LocationKind>, right: usize, down: usize) -> Result<usize, &'static str> {
	let mut j = 0;
	let mut n_trees = 0;

	for i in (0..grid.num_rows()).step_by(down) {
		let pos = grid[(i, j % grid.num_columns())];
		match pos {
			LocationKind::Tree => n_trees += 1,
			LocationKind::OpenSquare => (),
		}
		j += right;
	}

	Ok(n_trees)
}

fn solve_main_problem(filename: &String) -> Result<usize, &'static str> {
	let grid = parse_grid(filename).unwrap();
	let n_trees = solve_for_slope(&grid, 3, 1).unwrap();

	Ok(n_trees)
}

fn solve_aux_problem(filename: &String) -> Result<usize, &'static str> {
	let grid = parse_grid(filename).unwrap();
	let pairs = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

	let mut prod = 1;

	for (right, down) in pairs {
		let n_trees = solve_for_slope(&grid, right, down).unwrap();
		prod *= n_trees;
	};

	Ok(prod)
}
