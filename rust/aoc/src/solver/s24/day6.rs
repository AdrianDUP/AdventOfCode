use std::collections::HashMap;

use crate::solver::solver::{split_string_into_characters, Solver};

pub struct Day6 {
}

impl Solver for Day6 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let mut answer = 0;

        let grid = get_grid(lines);

        

        return 0;
    }

    fn solution_two(&self, lines: Vec<String>) -> i64 {
        return 0;
    }
}

fn get_grid(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut grid: Vec<Vec<String>> = vec![];

    for line in lines {
        grid.push(split_string_into_characters(line));
    }

    return grid;
}

fn find_start(grid: Vec<Vec<String>>) -> (i64, i64) {
    for (y, element) in grid.iter().enumerate() {
        for (x, element2) in element.iter().enumerate() {
            if element == "^" {
                return (x,y);
            }
        }
    }
}
