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

        let mut antennas: HashMap<String, Vec<Coordinate>> = HashMap::new();

        for ((row,col), item) in grid.indexed_iter() {
            if item == "." {
                continue;
            }

            let coord = Coordinate{
                row: row.try_into().unwrap(),
                col: col.try_into().unwrap()
            };

            antennas.entry(item.to_string()).and_modify(|e| e.push(coord)).or_insert(vec![coord]);
        }

        // dbg!(grid.clone());

        let mut antinodes: Vec<String> = vec![];

        for (_char,coordinate_list) in antennas.iter() {
            // dbg!(coordinate_list);
            for i in 0..coordinate_list.len() { 
                if i == coordinate_list.len() - 1 {
                    continue;
                }
                let coordinate_one = coordinate_list[i as usize];
                for j in i+1..coordinate_list.len() {
                    let coordinate_two = coordinate_list[j];
                    let (next_point_one, next_point_two) = get_next_points(coordinate_one, coordinate_two);
                    // dbg!(next_point_one, next_point_two);

                    if next_point_one.row < grid.rows() as i64 && next_point_one.row >= 0 && next_point_one.col < grid.cols() as i64 && next_point_one.col >= 0 {
                        let coordinate = format!("{},{}", next_point_one.row, next_point_one.col);
                        if !antinodes.contains(&coordinate) {
                            antinodes.push(coordinate);
                        }
                    }

                    if next_point_two.row < grid.rows() as i64 && next_point_two.row >= 0 && next_point_two.col < grid.cols() as i64 && next_point_two.col >= 0 {
                        let coordinate = format!("{},{}", next_point_two.row, next_point_two.col);
                        if !antinodes.contains(&coordinate) {
                            antinodes.push(coordinate);
                        }
                    }
                }
            }
        }

        dbg!(&antinodes);

        return antinodes.len() as i64;
    }

    fn solution_two(&self, lines: Vec<String>) -> i64 {
        let grid = get_grid(lines);

        let mut antennas: HashMap<String, Vec<Coordinate>> = HashMap::new();

        for ((row,col), item) in grid.indexed_iter() {
            if item == "." {
                continue;
            }

            let coord = Coordinate{
                row: row.try_into().unwrap(),
                col: col.try_into().unwrap()
            };

            antennas.entry(item.to_string()).and_modify(|e| e.push(coord)).or_insert(vec![coord]);
        }

        let mut antinodes: Vec<String> = vec![];
         
        for (_char, coordinate_list) in antennas.iter() {
            dbg!("iter");
            for i in 0..coordinate_list.len() {
                if i == coordinate_list.len() - 1 {
                    continue;
                }
                
                let coordinate_one = coordinate_list[i];

                for j in i..coordinate_list.len() {
                    let coordinate_two = coordinate_list[j];

                    let diff_col = coordinate_two.col - coordinate_one.col;
                    let diff_row = coordinate_two.row - coordinate_one.row;
                    
                    let mut one_in_bound: bool = true;
                    let mut two_in_bound: bool = true;

                    let mut new_one: Coordinate = coordinate_one;
                    let mut new_two: Coordinate = coordinate_two;
                    
                    while one_in_bound || two_in_bound {
                        dbg!("While");
                        if one_in_bound {
                            dbg!("one");
                            new_one.row -= diff_row;
                            new_one.col -= diff_col;

                            if new_one.row >= 0 && new_one.row < grid.rows() as i64 && new_one.col >= 0 && new_one.col < grid.cols() as i64 {
                                let cord = format!("{},{}", new_one.row, new_one.col);
                                dbg!(&cord);
                                if !antinodes.contains(&cord) {
                                    antinodes.push(cord);
                                }
                            } else {
                                one_in_bound = false;
                            }
                        }
                        if two_in_bound {
                            dbg!("two");
                            new_two.row += diff_row;
                            new_two.row += diff_row;

                            if new_two.row >= 0 && new_two.row < grid.rows() as i64 && new_two.col >= 0 && new_two.col < grid.cols() as i64 {
                                let cord = format!("{},{}", new_two.row, new_two.col);
                                dbg!(&cord);
                                if !antinodes.contains(&cord) {
                                    antinodes.push(cord);
                                }
                            } else {
                                two_in_bound = false;
                            }
                        }
                    }
                }
            }
        }

        return antinodes.len().try_into().unwrap();
    }
}

fn get_grid(lines: Vec<String>) -> Grid<String> {
    let mut grid: Grid<String> = grid![];

    for line in lines {
        grid.push_row(split_string_into_characters(line));
    }

    return grid;
}

fn get_next_points(coordinate_one: Coordinate, coordinate_two: Coordinate) -> (Coordinate, Coordinate) {
    // dbg!(coordinate_one, coordinate_two);
    let diff_col = coordinate_two.col - coordinate_one.col;
    let diff_row = coordinate_two.row - coordinate_one.row;
    // dbg!(diff_row, diff_col);

    let new_coordinate_one: Coordinate = Coordinate { row: coordinate_one.row - diff_row, col: coordinate_one.col - diff_col };
    let new_coordinate_two: Coordinate = Coordinate { row: coordinate_two.row + diff_row, col: coordinate_two.col + diff_col };

    return (new_coordinate_one, new_coordinate_two);
}
