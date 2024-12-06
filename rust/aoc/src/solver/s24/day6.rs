use std::collections::HashMap;

use crate::solver::solver::{split_string_into_characters, Solver};

pub struct Day6 {
}

impl Solver for Day6 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let mut moves: u64 = 0;
        let mut positions: HashMap<String, u8> = HashMap::new();

        let grid = get_grid(lines);
        dbg!(&grid);

        let (mut x, mut y) = find_start(grid.clone());

        dbg!(x, y);

        let mut direction = "up";

        while x >= 0 && x < grid[0].len().try_into().unwrap() && y >= 0 && y < grid.len().try_into().unwrap() {
            dbg!(moves);
            let (next_x, next_y) = next_pos(x, y, direction);

            let next_element: String = grid[y as usize][x as usize].clone();

            if next_element == "#" {
                direction = turn(direction);
            } else {
                let position = format!("{x},{y}");
                positions.entry(position).or_insert(1);
                x = next_x;
                y = next_y;
                moves += 1;
            }
        }
        return positions.len().try_into().unwrap();
    }

    fn solution_two(&self, _lines: Vec<String>) -> i64 {
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
    for y in 0..grid.len()-1 {
        for x in 0..grid[y].len()-1 {
            if grid[y][x] == "^" {
                return (x.try_into().unwrap(),y.try_into().unwrap());
            }
        }
    }

    panic!("Start not found");
}

fn turn(current_direction: &str) -> &str {
    return match current_direction {
        "up" => "right",
        "right" => "down",
        "down" => "left",
        "left" => "up",
        _ => panic!("nothing found"),
    };
}

fn next_pos(x: i64, y: i64, direction: &str) -> (i64, i64) {
    return match direction {
        "up" => (x, y-1),
        "right" => (x+1,y),
        "down" => (x,y-1),
        "left" => (x-1,y),
        _ => (x,y),
    };
}
