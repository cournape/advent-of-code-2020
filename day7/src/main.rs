#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

use regex::Regex;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("day 7")
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
			// let n_answers = solve_problem(&filename, &count_any_answer);
			solve_problem(&filename);
			// println!("{} answers", n_answers);
		},
		1 | _ => {
            panic!("Not implemented");
		}
	}
}

// const MY_BAG : String = "shiny gold".to_string();

fn solve_problem(filename: &String) -> () {
	let file = File::open(filename).expect("failed to open");

	let buf = io::BufReader::new(file);

    let mut parent_to_children = HashMap::new();
    let mut children_to_parents = HashMap::<String, Vec<String>>::new();

	for line in buf.lines() {
		let line = line.unwrap();
        let (parent, children) = parse_line(&line);
        for child in children.keys() {
            if !children_to_parents.contains_key(child) {
                children_to_parents.insert(child.to_string(), vec![parent.clone()]);
            } else {
                children_to_parents[child].push(parent);
            }
        }
        parent_to_children.insert(parent, children);
    }

    // println!("Found {} nodes", nodes.len());
    // let mut visited = HashSet::new();
    // let mut ancestors = HashSet::new();

    // while true {
    //     for (parent, children) in graph {
    //         if children.contains_key(parent) {
    //             ancestors.insert(parent)
    //         }
    //     }
    // }
}

const BAG_CONTAIN: &str = "bags contain";

fn parse_line(line: &String) -> (String, HashMap<String, i32>) {
	lazy_static! {
		static ref CHILD_BAG_RE: Regex = Regex::new(r"(\d+) (\w+ \w+) bag").unwrap();
	}

    let pos = line.find(BAG_CONTAIN).unwrap();

    let left_part = &line[..pos].trim();
    let right_part = line[pos + BAG_CONTAIN.len()..].trim();

    let mut children = HashMap::<String, i32>::new();

    if right_part != "no other bags." {
        for cap in CHILD_BAG_RE.captures_iter(right_part) {
            children.insert((&cap[2]).to_string(), cap[1].parse().unwrap());
        }
    }

    (left_part.to_string(), children)
}
