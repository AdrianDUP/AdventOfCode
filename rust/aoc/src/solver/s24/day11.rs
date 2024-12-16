use crate::solver::solver::Solver;

pub struct Day11 {
}

impl Solver for Day11 {
    fn solution_one(&self, _lines: Vec<String>) -> i64 {
        return 0;
    }

    fn solution_two(&self, _lines: Vec<String>) -> i64 {
        return 0;
    }
}

fn handle_stone_zero() -> i64 {
    return 1;
}

fn handle_even_stone(number: i64) -> (i64, i64) {
    let number_string: String = number.to_string();
    let half_way: usize = number_string.len() / 2;
}
