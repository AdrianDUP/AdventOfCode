use std::{collections::HashMap, env, fs, iter::Map};

use aoc::solver::solver::Solver;

fn main() {
    let args: Vec<String> = env::args().collect();
    let solvers: HashMap<u16, Vec<Box<dyn Solver>>> = get_solvers();

    if args.len() != 4 {
        println!("Arguments do not equal 4");
    }

    let year: u16 = args.get(1).unwrap().parse::<u16>().unwrap();
    let day: u8 = args.get(2).unwrap().parse::<u8>().unwrap();
    let part: u8 = args.get(3).unwrap().parse::<u8>().unwrap();

    let file_lines = read_file_lines(year, day);

    println!("Hello, world!");
}

fn read_file_lines(year: u16, day: u8) -> Vec<String> {
    let file_path = format!("inputs/{year}/{day}.txt");

    return fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}

fn get_solvers() -> HashMap<u16, Vec<Box<dyn Solver>>> {
    let mut solvers: HashMap<u16, Vec<Box<dyn Solver>>> = HashMap::new();

    return solvers;
}
