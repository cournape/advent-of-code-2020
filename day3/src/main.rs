use std::fs::File;
use std::io::{self, BufRead};

enum LocationKind{
	Tree,
	OpenSquare,
}
	
fn main() {
	let path = "./simple";
	let n_trees = solve_main_problem(&path.to_string()).unwrap();

	println!("Encountered {} trees", n_trees);
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
		for row in data {
			if row.len() != n_cols {
				panic!("Inconsistent grid");
			}
		}
	}

	Ok(0)
}
