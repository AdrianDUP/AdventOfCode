use std::collections::HashMap;

use crate::solver::solver::{split_string_into_characters, Solver};

pub struct Day6 {
}

impl Solver for Day6 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let mut positions: HashMap<String, u8> = HashMap::new();

        let grid = get_grid(lines);
        dbg!(&grid);

        let (mut x, mut y) = find_start(grid.clone());

        positions.entry(format!("{x},{y}")).or_insert(1);

        dbg!(x, y);

        let mut direction = "up";

        let row_limit: i64 = grid.len().try_into().unwrap();
        let column_limit: i64 = grid[0].len().try_into().unwrap();

        dbg!(row_limit, column_limit);

        while x > 0 && x < column_limit-1 && y > 0 && y < row_limit-1 {
            let (next_x, next_y) = next_pos(x, y, direction);
            dbg!(next_x, next_y);

            let next_row: Vec<String> = grid[next_y as usize].clone();
            let next_element: String = next_row[next_x as usize].clone();
            dbg!(&next_element);

            if next_element == "#" {
                dbg!("Turning");
                direction = turn(direction);
                dbg!(direction);
            } else {
                dbg!("Logging position");
                let position = format!("{x},{y}");
                positions.entry(position).or_insert(1);
                x = next_x;
                y = next_y;
            }
        }

        return positions.len().try_into().unwrap();
    }

    fn solution_two(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;

        let grid = get_grid(lines);

        let barries

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
        "down" => (x,y+1),
        "left" => (x-1,y),
        _ => (x,y),
    };
}

fn hits_barrier(grid: Vec<Vec<String>>, x: i64, y: i64, direction: &str) -> bool {
    if direction == "up" && y == 0 {
        return false;
    } else if direction == "right" && x == grid[0].len() as i64 - 1 {
        return false;
    } else if direction == "down" && y == grid.len() as i64 - 1 {
        return false;
    } else if direction == "left" && x == 0 {
        return false;
    }

    let (next_x,next_y) = next_pos(x, y, direction);
    let element = grid[next_y as usize][next_x as usize].clone();

    if element == "#" {
        return true;
    } else {
        return hits_barrier(grid, next_x, next_y, direction);
    }
}
