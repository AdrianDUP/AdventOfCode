use std::{collections::HashMap, env, fs};

use aoc::solver::{s23, s24, solver::Solver};

fn main() {
    let args: Vec<String> = env::args().collect();
    let solvers: HashMap<u16, HashMap<u8, Box<dyn Solver>>> = get_solvers();

    if args.len() < 4 {
        println!("Arguments do not equal 4");
    }

    let year: u16 = args.get(1).unwrap().parse::<u16>().unwrap();
    let day: u8 = args.get(2).unwrap().parse::<u8>().unwrap();
    let part: u8 = args.get(3).unwrap().parse::<u8>().unwrap();
    let test:bool;

    if args.len() == 5 {
        test = true;
    } else {
        test = false;
    }

    let file_lines = read_file_lines(year, day, test);

    dbg!(year, day, part, test);

    let solver = solvers.get(&year).unwrap().get(&day).unwrap();

    let answer = match part {
        1 => solver.solution_one(file_lines),
        2 => solver.solution_two(file_lines),
        _ => panic!("Invalid Part")
    };

    println!("Answer for {year} day {day} part {part} is {answer}");
}

fn read_file_lines(year: u16, day: u8, test: bool) -> Vec<String> {
    let file_path: String;
    if test {
        file_path = format!("./inputs/{year}/day{day}_test.txt");
    } else {
        file_path = format!("./inputs/{year}/day{day}.txt");
    }

    return fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}

fn get_solvers() -> HashMap<u16, HashMap<u8, Box<dyn Solver>>> {
    let mut solvers: HashMap<u16, HashMap<u8, Box<dyn Solver>>> = HashMap::new();

    let mut s23_solvers: HashMap<u8, Box<dyn Solver>> = HashMap::new();

    s23_solvers.insert(1, Box::new(s23::day1::Day1{}));

    let mut s24_solvers: HashMap<u8, Box<dyn Solver>> = HashMap::new();

    s24_solvers.insert(1, Box::new(s24::day1::Day1{}));
    s24_solvers.insert(2, Box::new(s24::day2::Day2{}));
    s24_solvers.insert(3, Box::new(s24::day3::Day3{}));
    s24_solvers.insert(4, Box::new(s24::day4::Day4{}));
    s24_solvers.insert(5, Box::new(s24::day5::Day5{}));
    s24_solvers.insert(6, Box::new(s24::day6::Day6{}));
    s24_solvers.insert(7, Box::new(s24::day7::Day7{}));
    s24_solvers.insert(8, Box::new(s24::day8::Day8{}));
    // s24_solvers.insert(9, Box::new(s24::day9::Day9{}));
    // s24_solvers.insert(10, Box::new(s24::day10::Day10{}));
    s24_solvers.insert(11, Box::new(s24::day11::Day11{}));
    s24_solvers.insert(17, Box::new(s24::day17::Day17{}));
    s24_solvers.insert(20, Box::new(s24::day20::Day20{}));

    solvers.insert(2023, s23_solvers);
    solvers.insert(2024, s24_solvers);

    return solvers;
}
