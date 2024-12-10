use grid::{grid, Grid};

use crate::solver::solver::{split_string_into_characters, Solver};

pub struct Day10 {
}

impl Solver for Day10 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        return 0;
    }

    fn solution_two(&self, lines: Vec<String>) -> i64 {
        return 0;
    }
}

fn get_grid(lines: Vec<String>) -> Grid<String> {
    let mut grid: Grid<String> = grid![];

    for line in lines {
        let line_characters: Vec<String> = split_string_into_characters(line);
        grid.push_row(line_characters);
    }
    
    return grid;
}
