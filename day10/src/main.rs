use std::fs::File;
use std::io::{self, BufRead};

extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("day 10")
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
            println!("prod is: {}", value);
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

    values.sort();
    let mut diff = Vec::new();

    diff.push(values[0]);

    for i in 0..(values.len() - 1) {
        diff.push(values[i + 1] -  values[i]);
    }

    let mut count_1 = 0;
    let mut count_3 = 1;
    for v in diff {
        match v {
            1 => count_1 += 1,
            3 => count_3 += 1,
            _ => panic!("yolo"),
        }
    }

    count_1 * count_3
}
