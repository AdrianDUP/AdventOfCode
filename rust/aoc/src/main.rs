use std::{collections::HashMap, env, fs};

use aoc::solver::{s24, solver::Solver};

fn main() {
    let args: Vec<String> = env::args().collect();
    let solvers: HashMap<u16, HashMap<u8, Box<dyn Solver>>> = get_solvers();

    if args.len() != 4 {
        println!("Arguments do not equal 4");
    }

    let year: u16 = args.get(1).unwrap().parse::<u16>().unwrap();
    let day: u8 = args.get(2).unwrap().parse::<u8>().unwrap();
    let part: u8 = args.get(3).unwrap().parse::<u8>().unwrap();

    let file_lines = read_file_lines(year, day);

    let solver = solvers.get(&year).unwrap().get(&day).unwrap();

    let answer = match part {
        1 => solver.solution_one(file_lines),
        2 => solver.solution_two(file_lines),
        _ => panic!("Invalid Part")
    };

    println!("Answer for {year} day {day} part {part} is {answer}");
}

fn read_file_lines(year: u16, day: u8) -> Vec<String> {
    let file_path = format!("./inputs/{year}/day{day}.txt");

    return fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}

fn get_solvers() -> HashMap<u16, HashMap<u8, Box<dyn Solver>>> {
    let mut solvers: HashMap<u16, HashMap<u8, Box<dyn Solver>>> = HashMap::new();

    let mut s24_solvers: HashMap<u8, Box<dyn Solver>> = HashMap::new();

    s24_solvers.insert(1, Box::new(s24::day1::Day1{}));

    solvers.insert(2024, s24_solvers);

    return solvers;
}
