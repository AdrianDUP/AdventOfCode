use std::mem::replace;

use crate::solver::solver::{find_all_by_regex, Solver};


pub struct Day1{
}

impl Solver for Day1 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;

        for line in lines {
            let (left, right) = get_first_and_last_number(line);

            let final_string = format!("{left}{right}");
            answer += final_string.parse::<i64>().unwrap();
        }

        return answer;
    }

    fn solution_two(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;

        for line in lines {
            let modified_line = replace_illegal(line);
            let (left, right) = get_first_and_last_number(modified_line);
            let final_string = format!("{left}{right}");
            answer += final_string.parse::<i64>().unwrap();
        }

        return answer;
    }
}

fn get_first_and_last_number(line: String) -> (i64, i64) {
    let numbers = find_all_by_regex(line, r"[0-9]");

    let first_number = numbers[0].parse::<i64>().unwrap();
    let last_number = numbers[numbers.len()-1].parse::<i64>().unwrap();
    
    return (first_number, last_number);
}

fn replace_illegal(line: String) -> String {
    return line.replace("oneight", "18")
        .replace("nineight", "98")
        .replace("twone", "21")
        .replace("threeight", "38")
        .replace("sevenine", "79")
        .replace("eightwo", "82")
        .replace("eighthree", "83")
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
        .to_string();
}
