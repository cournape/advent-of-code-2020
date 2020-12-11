use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};

type Grid2D = Vec<Vec<LocationKind>>;
type ShouldOccupyCallback = dyn Fn(&Grid2D, usize, usize) -> bool;
type ShouldEmptyCallback = dyn Fn(&Grid2D, usize, usize) -> bool;

extern crate clap;
use clap::{App, Arg};

#[derive(Copy, Clone)]
enum LocationKind{
	Floor,
	Empty,
        Occupied,
}

struct Neighborhood {
    // neighborhood boundaries
    row_end: usize,
    col_begin: usize,
    col_end: usize,
    // neighborhood center
    center_i: usize,
    center_j: usize,
    // current position
    cur_i: usize,
    cur_j: usize,
}

impl Neighborhood {
    fn new(n_rows: usize, n_cols: usize, i: usize, j: usize) -> Neighborhood {
        let row_begin = cmp::max(i as isize - 1 , 0) as usize;
        let col_begin = cmp::max(j as isize - 1 , 0) as usize;
        Neighborhood {
            row_end: cmp::min(i + 2, n_rows),
            col_begin: col_begin,
            col_end: cmp::min(j + 2, n_cols),
            center_i: i,
            center_j: j,
            cur_i: row_begin,
            cur_j: col_begin,
        }
    }

    fn advance(&mut self) -> () {
        if self.cur_j == (self.col_end - 1) {
            self.cur_j = self.col_begin;
            self.cur_i += 1;
        } else {
            self.cur_j += 1
        }
    }
}

impl Iterator for Neighborhood {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let is_center = self.cur_i == self.center_i && self.cur_j == self.center_j;
        if is_center {
            self.advance();
            return self.next();
        }

        let has_reached_end = self.cur_i == self.row_end;
        if  has_reached_end {
            return None;
        } 

        let index = Some((self.cur_i, self.cur_j));
        self.advance();
        index
    }

}

fn main() {
    let matches = App::new("day 11")
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
            let count = solve_part(&filename, &should_occupy_part1, &should_empty_part1);
            println!("Found {} occupied seats", count);
        }
        1 | _ => {
            let count = solve_part(&filename, &should_occupy_part2, &should_empty_part2);
            println!("Found {} occupied seats", count);
        }
    }
}

fn parse_grid(filename: &String) -> Grid2D {
    let file = File::open(filename).expect("failed to open");
    let buf = io::BufReader::new(file);

    let mut grid = Grid2D::new();

    for line in buf.lines() {
        let line = line.unwrap();
        let mut line_data = Vec::<LocationKind>::new();
        for c in line.split("") {
            match c {
                "." => line_data.push(LocationKind::Floor),
                "#" => line_data.push(LocationKind::Occupied),
                "L" => line_data.push(LocationKind::Empty),
                "" => (),
                _ => panic!("Unexpected char '{}' ", c),
            }	
        }
        grid.push(line_data);
    }

    grid
}

// For debugging purpose
#[allow(dead_code)]
fn draw_grid(grid: &Grid2D) -> () {
    for row in grid.iter() {
        for element in row.iter() {
            let glyph = match element {
                LocationKind::Floor => ".",
                LocationKind::Occupied => "#",
                LocationKind::Empty => "L",
            };
            print!("{}", glyph);
        }
        println!();
    }
}

// If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes
// occupied.
fn should_occupy_part1(grid: &Grid2D, m: usize, n: usize) -> bool {
    let neigh = Neighborhood::new(grid.len(), grid[0].len(), m, n);
    for (i, j) in neigh {
        match grid[i][j] {
            LocationKind::Occupied => return false,
            _ => (),
        }
    }
    true
}

// If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat
// becomes empty.
fn should_empty_part1(grid: &Grid2D, m: usize, n: usize) -> bool {
    let neigh = Neighborhood::new(grid.len(), grid[0].len(), m, n);
    let mut occupied = 0;
    for (i, j) in neigh {
        match grid[i][j] {
            LocationKind::Occupied => occupied += 1,
            _ => (),
        }
        if occupied >= 4 {
            return true;
        }
    }
    false
}

fn should_occupy_part2(grid: &Grid2D, m: usize, n: usize) -> bool {
    let (_, occupied) = look_in_all_directions(&grid, m, n);
    return occupied == 0;
}

fn should_empty_part2(grid: &Grid2D, m: usize, n: usize) -> bool {
    let (_, occupied) = look_in_all_directions(&grid, m, n);
    return occupied >= 5;
}

// make change to grid in place
fn make_pass(grid: &mut Grid2D,
    should_occupy: &ShouldOccupyCallback, should_empty: &ShouldEmptyCallback,
    ) -> bool {
    let mut new_state = grid.clone();
    let mut has_changed = false;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                LocationKind::Empty => {
                    if should_occupy(&grid, i, j) {
                        has_changed = true;
                        new_state[i][j] = LocationKind::Occupied;
                    }
                },
                LocationKind::Occupied => {
                    if should_empty(&grid, i, j) {
                        has_changed = true;
                        new_state[i][j] = LocationKind::Empty;
                    } 
                },
                LocationKind::Floor => (),
            }
        }
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid[i][j] = new_state[i][j];
        }
    }

    has_changed
}

fn count_occupied_seats(grid: &Grid2D) -> usize {
    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                LocationKind::Occupied => {
                    count += 1
                },
                _ => (),
            }
        }
    }

    count
}

fn solve_part(filename: &String, should_occupy: &ShouldOccupyCallback, should_empty: &ShouldEmptyCallback) -> usize {
    let mut grid = parse_grid(&filename);

    for _i in 0..1000 {
        let has_changed = make_pass(&mut grid, should_occupy, should_empty);
        if !has_changed {
            return count_occupied_seats(&grid);
        }
    }
    panic!("over max number of iteration");
}

macro_rules! find_occupied_or_empty {
    ($match:expr, $a:expr, $b:expr) => {
        match $match {
            LocationKind::Occupied => {
                $a += 1;
                break;
            },
            LocationKind::Empty => {
                $b += 1;
                break;
            },
            _ => (),
        }
    };
}

fn look_in_all_directions(grid: &Grid2D, x: usize, y: usize) -> (usize, usize) {
    let mut empty = 0;
    let mut occupied = 0;

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    // relative to x, open limit
    let max_north = x + 1;
    // relative to x, open limit
    let max_south = n_rows - x;
    // relative to y, open limit
    let max_east = n_cols - y;
    // relative to y, open limit
    let max_west = y + 1;

    // North
    for j in 1..max_north {
        find_occupied_or_empty!(grid[x - j][y], occupied, empty);
    }

    // North-East
    for j in 1..cmp::min(max_north, max_east) {
        find_occupied_or_empty!(grid[x - j][y + j], occupied, empty);
    }

    // East
    for j in 1..max_east {
        find_occupied_or_empty!(grid[x][y + j], occupied, empty);
    }

    // South East
    for j in 1..cmp::min(max_south, max_east) {
        find_occupied_or_empty!(grid[x + j][y + j], occupied, empty);
    }

    // South
    for j in 1..max_south {
        find_occupied_or_empty!(grid[x + j][y], occupied, empty);
    }

    // South West
    for j in 1..cmp::min(max_south, max_west) {
        find_occupied_or_empty!(grid[x + j][y - j], occupied, empty);
    }

    // West
    for j in 1..max_west {
        find_occupied_or_empty!(grid[x][y - j], occupied, empty);
    }

    // North West
    for j in 1..cmp::min(max_west, max_north) {
        find_occupied_or_empty!(grid[x - j][y - j], occupied, empty);
    }

    return (empty, occupied);
}
