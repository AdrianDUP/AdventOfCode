use std::collections::HashMap;

use grid::{Grid, grid};

use crate::solver::solver::{split_string_into_characters, Solver};

pub struct Day8 {
}

#[derive(Copy, Clone, Debug)]
struct Coordinate {
    row: i64,
    col: i64
}

impl Solver for Day8 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let grid = get_grid(lines);

        let antennas: HashMap<String, Vec<Coordinate>> = HashMap::new();

        for ((row,col), item) in grid.indexed_iter() {
            if item == "." {
                continue;
            }

            let _coord = Coordinate{
                row: row.try_into().unwrap(),
                col: col.try_into().unwrap()
            };
        }

        let mut antinodes: Vec<String> = vec![];

        for (_char,coordinate_list) in antennas.iter() {
            for i in 0..coordinate_list.len() { 
                if i == coordinate_list.len() - 1 {
                    continue;
                }
                let coordinate_one = coordinate_list[i as usize];
                for j in i+1..coordinate_list.len() {
                    let coordinate_two = coordinate_list[j];
                    let (next_point_one, next_point_two) = get_next_points(coordinate_one, coordinate_two);
                    dbg!(next_point_one, next_point_two);

                    if next_point_one.row < grid.rows() as i64 && next_point_one.row > 0 && next_point_one.col < grid.cols() as i64 && next_point_one.col > 0 {
                        let coordinate = format!("{},{}", next_point_one.row, next_point_one.col);
                        if !antinodes.contains(&coordinate) {
                            antinodes.push(coordinate);
                        }
                    }

                    if next_point_two.row < grid.rows() as i64 && next_point_two.row > 0 && next_point_two.col < grid.cols() as i64 && next_point_two.col > 0 {
                        let coordinate = format!("{},{}", next_point_two.row, next_point_two.col);
                        if !antinodes.contains(&coordinate) {
                            antinodes.push(coordinate);
                        }
                    }
                }
            }
        }

        return antinodes.len() as i64;
    }

    fn solution_two(&self, _lines: Vec<String>) -> i64 {
        return 0;
    }
}

fn get_grid(lines: Vec<String>) -> Grid<String> {
    let mut grid: Grid<String> = grid![];

    for line in lines {
        let chars = split_string_into_characters(line.clone());
        grid.push_row(split_string_into_characters(line));
    }

    return grid;
}

fn get_next_points(coordinate_one: Coordinate, coordinate_two: Coordinate) -> (Coordinate, Coordinate) {
    let diff_col = coordinate_one.col - coordinate_two.col;
    let diff_row = coordinate_one.row - coordinate_two.row;
    dbg!(diff_row, diff_col);

    let new_coordinate_one: Coordinate;
    let new_coordinate_two: Coordinate;

    if diff_col > 0 && diff_row > 0 {
        new_coordinate_one = Coordinate{
            row: coordinate_one.row - diff_row,
            col: coordinate_one.col - diff_col
        };
        new_coordinate_two = Coordinate{
            row: coordinate_two.row + diff_row,
            col: coordinate_two.col + diff_row
        };
    } else {
        new_coordinate_one = Coordinate{
            row: coordinate_one.row + diff_row,
            col: coordinate_one.col + diff_col
        };
        new_coordinate_two = Coordinate{
            row: coordinate_two.row - diff_row,
            col: coordinate_two.col - diff_row
        };
    }

    return (new_coordinate_one,new_coordinate_two);
}
