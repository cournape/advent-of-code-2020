use std::fs::File;
use std::io::{self, BufRead};
// use std::path::Path;

fn main() {
	let filename = "./input";
	let file = File::open(filename).expect("failed to open");

	let buf = io::BufReader::new(file);

	let mut vec = Vec::<u32>::new();

	for line in buf.lines() {
		let value: u32 = line.unwrap().parse().unwrap();
		vec.push(value)
	}

	println!("Read {} values", vec.len());
	println!("Sum is {}", vec.iter().sum::<u32>());

	let (left, right) = find_pair(&vec).unwrap();

	println!("Pair Solution is {}", left * right);

	let (left, middle, right) = find_triplet(&vec).unwrap();
	println!("Triplet Solution is {}", left * middle * right);
}

fn find_pair(vec: &Vec::<u32>) -> Result<(u32, u32), &'static str> {
	for &left in vec {
		for &right in vec {
			if left + right == 2020 {
				return Ok((left, right))
			}
		}
	}

	Err("No pair found")
}

fn find_triplet(vec: &Vec::<u32>) -> Result<(u32, u32, u32), &'static str> {
	for &left in vec {
		for &middle in vec {
			for &right in vec {
				if left + middle + right == 2020 {
					return Ok((left, middle, right))
				}
			}
		}
	}

	Err("No triplet found")
}
