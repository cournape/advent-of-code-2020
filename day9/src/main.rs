use std::fs::File;
use std::io::{self, BufRead};

extern crate clap;
use clap::{App, Arg};
use itertools::Itertools;

const PREAMBULE_SIZE: usize = 25;

fn main() {
    let matches = App::new("day 8")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("part-two")
                .short("2")
                .long("part-two")
                .help("If set, solves part 2"),
        )
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap().to_string();

    match matches.occurrences_of("part-two") {
        0 => {
            let value = solve_part1(&filename);
            println!("First non valid number: {}", value);
        }
        1 | _ => {
            panic!("not implemented yet");
        }
    }
}

fn solve_part1(filename: &String) -> isize {
    let file = File::open(filename).expect("failed to open");
    let buf = io::BufReader::new(file);

    let mut values: Vec<isize> = Vec::new();

    for line in buf.lines() {
        let value = line.unwrap().parse().unwrap();
        values.push(value);
    }

    println!("Parsed {} entries", values.len());

    if PREAMBULE_SIZE >= values.len() {
        panic!(
            "{} entries, need at least {}",
            values.len(),
            PREAMBULE_SIZE + 1
        );
    }

    for i in 0..(values.len() - PREAMBULE_SIZE) {
        let start = i;
        let end = start + PREAMBULE_SIZE;

        if !can_find_pair(&values[start..end + 1]) {
            return values[end];
        }
    }
    panic!("Could not find invalid pair");
}

fn can_find_pair(v: &[isize]) -> bool {
    let value = v[v.len() - 1];
    let it = (0..v.len() - 1).cartesian_product(0..v.len() - 1);

    for (i, j) in it {
        if (i != j) && (v[i] + v[j] == value) {
            // println!("Could find pair for {}: {}, {}", value, v[i], v[j]);
            return true;
        }
    }

    false
}
