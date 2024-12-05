use crate::solver::solver::{split_string_into_characters, Solver};

pub struct Day3 {
}

impl Solver for Day3 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let answer: i64 = 0;

        for (_index,line) in lines.iter().enumerate() {
            let _characters = split_string_into_characters(line.to_string());

        }
        return answer;
    }

    fn solution_two(&self, _lines: Vec<String>) -> i64 {
        todo!()
    }
}
