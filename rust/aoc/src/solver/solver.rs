use grid::{grid, Grid};
use nalgebra::Point2;

pub trait Solver {
    fn solution_one(&self, lines: Vec<String>) -> i64;
    fn solution_two(&self, lines: Vec<String>) -> i64;
}

pub fn find_all_by_regex(input: String, regex: &str) -> Vec<String> {
    let regx = regex::Regex::new(regex).unwrap();
    
    return regx.find_iter(&input).map(|e| String::from(e.as_str())).collect();
}

pub fn split_string_into_characters(line: String) -> Vec<String> {
    return line.split("")
        .filter(|e| *e != "")
        .map(String::from)
        .collect();
}

pub fn split_string_into_parts(line: String) -> Vec<String> {
    return line.split_whitespace()
        .filter(|x| *x != "")
        .map(String::from)
        .collect();
}

pub fn get_grid(lines: Vec<String>) -> Grid<String> {
    let mut grid: Grid<String> = grid![];
    
    for line in lines {
        grid.push_row(split_string_into_characters(line));
    }

    return grid;
}

pub fn get_next_point(mut coordinate: Point2<i64>, direction: &str) -> Point2<i64> {
    let x_modifier: i64;
    let y_modifier: i64;

    if direction == "up" {
        x_modifier = 0;
        y_modifier = -1;
    } else if direction == "right" {
        x_modifier = 1;
        y_modifier = 0;
    } else if direction == "down" {
        x_modifier = 0;
        y_modifier = 1;
    } else if direction == "left" {
        x_modifier = -1;
        y_modifier = 0;
    } else {
        panic!("Invalid direction given");
    }

    coordinate.x += x_modifier; 
    coordinate.y += y_modifier; 

    return coordinate;
}

pub fn turn_left(direction: &str) -> &str {
    return match direction {
        "up" => "left",
        "left" => "down",
        "down" => "right",
        "right" => "up",
        _ => "none",
    };
}

pub fn turn_right(direction: &str) -> &str {
    return match direction {
        "up" => "right",
        "left" => "up",
        "down" => "left",
        "right" => "down",
        _ => "none",
    };
}

pub fn point_within_grid_bounds(grid: Grid<String>, coordinate: Point2<i64>) -> bool {
    return coordinate.x > 0 && coordinate.y > 0 && coordinate.x < grid.cols() as i64 && coordinate.y < grid.rows() as i64;
}
