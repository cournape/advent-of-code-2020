use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

extern crate clap;
use clap::{App, Arg};

#[derive(Clone, Copy)]
enum Opcode {
    AccKind(isize),
    JumpKind(isize),
    NopKind(isize),
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
            let acc = solve_problem_1(&filename);
            println!("Value of acc at termination: {}", acc);
        }
        1 | _ => {
            let acc = solve_problem_2(&filename);
            println!("Value of acc at termination: {}", acc);
        }
    }
}

fn solve_problem_1(filename: &String) -> isize {
    let instructions = parse_instructions(filename);
    let (acc, _) = run_vm(&instructions);
    acc
}

fn solve_problem_2(filename: &String) -> isize {
    let instructions = parse_instructions(filename);
    let mut instructions = instructions.clone(); 
    let n_instructions = instructions.len();

    for i in 0..n_instructions {
        let old_opcode = instructions[i];
        match old_opcode {
            Opcode::NopKind(value) => {
                instructions[i] = Opcode::JumpKind(value);
                let (acc, ip) = run_vm(&instructions);
                instructions[i] = old_opcode;
                if ip == n_instructions {
                    return acc;
                }
            },
            Opcode::JumpKind(value) => {
                instructions[i] = Opcode::NopKind(value);
                let (acc, ip) = run_vm(&instructions);
                instructions[i] = old_opcode;
                if ip == n_instructions {
                    return acc;
                }
            },
            Opcode::AccKind(_) => (),
        };
    }

    panic!("Could not fix instructions !");
}

fn run_vm(instructions: &Vec<Opcode>) -> (isize, usize) {
    let n_instructions = instructions.len();

    let mut acc: isize = 0;
    let mut ip: usize = 0;
    let mut visited = HashSet::new();

    while !visited.contains(&ip) {
        if ip > n_instructions {
            panic!("Overflow !");
        } else if ip == n_instructions {
            return (acc, ip);
        }

        visited.insert(ip);

        let opcode = &instructions[ip];

        match opcode {
            Opcode::AccKind(value) => {
                ip += 1;
                acc += value;
            }
            Opcode::NopKind(_) => {
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

    (acc, ip)
}

fn parse_instructions(filename: &String) -> Vec<Opcode> {
    let file = File::open(filename).expect("failed to open");

    let buf = io::BufReader::new(file);
    let mut instructions = Vec::new();

    for line in buf.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let opcode_string = iter.next().unwrap();
        let value = iter.next().unwrap().parse().unwrap();
        let opcode = match opcode_string {
            "acc" => Opcode::AccKind(value),
            "jmp" => Opcode::JumpKind(value),
            "nop" => Opcode::NopKind(value),
            _ => panic!("Unknown opcode: {}", opcode_string),
        };
        instructions.push(opcode);
    }

    instructions
}
