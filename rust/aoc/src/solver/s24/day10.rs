use grid::{grid, Grid};
use nalgebra::Point2;

use crate::solver::solver::{get_next_point, point_within_grid_bounds, split_string_into_characters, turn_right, Solver};

pub struct Day10 {
}

impl Solver for Day10 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let grid = get_grid(lines);
        let starts = get_trailheads(grid);
        dbg!(starts);
        return 0;
    }

    fn solution_two(&self, _lines: Vec<String>) -> i64 {
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

fn get_trailheads(grid: Grid<String>) -> Vec<Point2<i64>> {
    let mut coordinates: Vec<Point2<i64>> = vec![];

    for ((row, column), item) in grid.indexed_iter() {
        if *item == "0".to_string() {
            coordinates.push(Point2::new(column.try_into().unwrap(), row.try_into().unwrap()));
        }
    }

    return coordinates;
}

fn follow_path(grid: Grid<String>, coordinate: Point2<i64>, mut direction: &str) -> i64 {
    if direction == "none" {
        //  No direction has been set, now we need to get the initial direction
        if coordinate.y == 0 {
            direction = "right";
        } else {
            direction = "up";
        }
        
        let mut temp_next_element: Point2<i64> = get_next_point(coordinate, direction);
        if !point_within_grid_bounds(grid, temp_next_element) {
            direction = turn_right(direction);
            temp_next_element = get_next_point(coordinate, direction)
        }
        if !point_within_grid_bounds(grid, temp_next_element) {
            direction = turn_right(direction);
            temp_next_element = get_next_point(coordinate, direction)
        }
        if !point_within_grid_bounds(grid, temp_next_element) {
            direction = turn_right(direction);
            temp_next_element = get_next_point(coordinate, direction)
        }
        if !point_within_grid_bounds(grid, temp_next_element) {
            direction = turn_right(direction);
            temp_next_element = get_next_point(coordinate, direction)
        }
        if !point_within_grid_bounds(grid, temp_next_element) {
            return 0;
        }
        
        if grid.get(temp_next_element.y, temp_next_element.x).unwrap() != "1" {
            
        }
    }
    
    let current_element = grid.get(coordinate.y, coordinate.x).unwrap();

    let next_number = next_number(current_element.to_string());
}

fn next_number(number: String) -> String {
    return match number.as_str() {
        "0" => "1".to_string(),
        "1" => "2".to_string(),
        "2" => "3".to_string(),
        "3" => "4".to_string(),
        "4" => "5".to_string(),
        "5" => "6".to_string(),
        "6" => "7".to_string(),
        "7" => "8".to_string(),
        "8" => "9".to_string(),
        "9" => "".to_string(),
        _ => panic!("This should not happen"),
    };
}
