use std::collections::{HashMap, HashSet};

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
        let grid = get_grid(lines);
        let (sx, sy) = find_start(grid.clone());

        let row_limit: i64 = grid.len() as i64;
        let col_limit: i64 = grid[0].len() as i64;

        let mut count: i64 = 0;

        for y in 0..row_limit {
            for x in 0..col_limit {
                // Can't place on the starting position and must be an empty cell
                if x == sx && y == sy { continue; }
                if grid[y as usize][x as usize] != "." { continue; }

                if simulate_loop(&grid, (sx, sy), (x, y)) {
                    count += 1;
                }
            }
        }

        count
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


fn simulate_loop(grid: &Vec<Vec<String>>, start: (i64, i64), obstacle: (i64, i64)) -> bool {
    let (sx, sy) = start;
    let (ox, oy) = obstacle;
    let rows: i64 = grid.len() as i64;
    let cols: i64 = grid[0].len() as i64;

    let mut x = sx;
    let mut y = sy;
    let mut dir: i8 = 0; // 0=up,1=right,2=down,3=left

    let mut seen: HashSet<(i64, i64, i8)> = HashSet::new();
    seen.insert((x, y, dir));

    loop {
        let (nx, ny) = match dir {
            0 => (x, y - 1),
            1 => (x + 1, y),
            2 => (x, y + 1),
            _ => (x - 1, y),
        };

        if nx < 0 || ny < 0 || nx >= cols || ny >= rows {
            return false; // exited the grid
        }

        let blocked = (nx == ox && ny == oy) || grid[ny as usize][nx as usize] == "#";
        if blocked {
            dir = (dir + 1) % 4;
        } else {
            x = nx;
            y = ny;
        }

        if !seen.insert((x, y, dir)) {
            return true; // loop detected
        }
    }
}
