use std::fs::File;
use std::io::{self, BufRead};

extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("day 13")
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
            let sol = solve_problem1(&filename);
            println!("Solution part 1: {}", sol);
        }
        1 | _ => {
            panic!("Not implemented yet");
        }
    }
}

fn solve_problem1(filename: &String) -> usize {
    let (start_time, bus_ids) = parse_data(&filename);
    let modulos: Vec<usize> = bus_ids.iter().map(|bus_id| bus_id - start_time % bus_id).collect();

    // argmin
    if modulos.len() < 1 {
        panic!("Unexpected vector size");
    }

    let mut min = modulos[0];
    let mut i_min = 0;
    for (i, val) in modulos.iter().enumerate() {
        if *val < min {
            min = *val;
            i_min = i;
        }
    }

    let first_bus_id = bus_ids[i_min];

    first_bus_id * min
}

fn parse_data(filename: &String) ->  (usize, Vec<usize>) {
    let file = File::open(filename).expect("failed to open");
    let buf = io::BufReader::new(file);

    let data: Vec<String>  = buf.lines().map(|x| x.unwrap()).collect();

    if data.len() != 2 {
        panic!("Read {} lines, expected 2", data.len());
    }

    let start_time: usize = data[0].parse().unwrap();

    let bus_ids: Vec<usize> = data[1]
        .split(',')
        .filter(|x| x != &"x")
        .map(|x| x.parse().unwrap())
        .collect();

    (start_time, bus_ids)
}
