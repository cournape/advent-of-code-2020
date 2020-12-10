use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

extern crate clap;
use clap::{App, Arg};

enum Opcode {
    AccKind(isize),
    JumpKind(isize),
    NopKind,
}

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
    println!("Using input file: {}", filename);

    match matches.occurrences_of("part-two") {
        0 => {
            let acc = solve_problem(&filename);
            println!("Value of acc at termination: {}", acc);
        }
        1 | _ => {
            panic!("Not implemented");
        }
    }
}

fn solve_problem(filename: &String) -> isize {
    let instructions = parse_instructions(filename);
    run_vm(instructions)
}

fn run_vm(instructions: Vec<Opcode>) -> isize {
    let mut acc: isize = 0;
    let mut ip: usize = 0;
    let mut visited = HashSet::new();

    while !visited.contains(&ip) {
        visited.insert(ip);

        let opcode = &instructions[ip];

        match opcode {
            Opcode::AccKind(value) => {
                ip += 1;
                acc += value;
            }
            Opcode::NopKind => {
                ip += 1;
            }
            Opcode::JumpKind(jump) => {
                let tmp_ip = ip as isize + jump;
                if tmp_ip < 0 {
                    panic!("Found negative ip !");
                }
                ip = tmp_ip as usize;
            }
        };
    }

    acc
}

fn parse_instructions(filename: &String) -> Vec<Opcode> {
    let file = File::open(filename).expect("failed to open");

    let buf = io::BufReader::new(file);
    let mut instructions = Vec::new();

    for line in buf.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let opcode_string = iter.next().unwrap();
        let opcode = match opcode_string {
            "acc" => Opcode::AccKind(iter.next().unwrap().parse().unwrap()),
            "jmp" => Opcode::JumpKind(iter.next().unwrap().parse().unwrap()),
            "nop" => Opcode::NopKind,
            _ => panic!("yolo"),
        };
        instructions.push(opcode);
    }

    instructions
}
