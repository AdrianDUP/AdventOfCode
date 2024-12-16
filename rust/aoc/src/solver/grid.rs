use grid::{self, grid};
use nalgebra::Point2;

use super::solver::split_string_into_characters;

pub struct Grid<T> {
    grid: grid::Grid<T>,
    current_position: Point2<i64>
}

impl<T> Grid<T> {
    fn set_position(&mut self, coordinate: Point2<i64>) {
        if !self.is_point_in_bounds(coordinate) {
            panic!("Invalid coordinate, out of bounds");
        }
        self.current_position = coordinate;
    }

    fn is_point_in_bounds(&self, coordinate: Point2<i64>) -> bool {
        return coordinate.x >= 0 && coordinate.y >= 0 && coordinate.x < self.grid.cols() as i64 && coordinate.y < self.grid.rows() as i64;
    }

    fn get_coordinate(&self, coordinate: Point2<i64>) -> &T {
        return self.grid.get(coordinate.y, coordinate.x).unwrap();
    }

    fn get_relative_position(&self, direction: Direction) -> &T {
        return match direction {
            Direction::UP => self.position_up().unwrap(),
            Direction::RIGHT => self.position_right().unwrap(),
            Direction::DOWN => self.position_down().unwrap(),
            Direction::LEFT => self.position_left().unwrap(),
        }
    }

    fn get_relative_coordinate(&self, direction: Direction) -> Point2<i64> {
        return match direction {
            Direction::UP => self.get_relative_coordinate_up(),
            Direction::RIGHT => self.get_relative_coordinate_right(),
            Direction::DOWN => self.get_relative_coordinate_down(),
            Direction::LEFT => self.get_relative_coordinate_left(),
        }
    }

    fn get_relative_coordinate_up(&self) -> Point2<i64> {
        let mut position = self.current_position;
        position.y -= 1;

        return position;
    }

    fn get_relative_coordinate_down(&self) -> Point2<i64> {
        let mut position = self.current_position;

        position.y += 1;

        return position;
    }

    fn get_relative_coordinate_right(&self) -> Point2<i64> {
        let mut position = self.current_position;

        position.x += 1;

        return position;
    }

    fn get_relative_coordinate_left(&self) -> Point2<i64> {
        let mut position = self.current_position;

        position.x -= 1;

        return position;
    }
 
    fn position_up(&self) -> Option<&T> {
        let next_coordinate = self.get_relative_coordinate_up();

        if self.is_point_in_bounds(next_coordinate) {
            return Some(self.get_coordinate(next_coordinate));
        }

        return None
    }
 
    fn position_right(&self) -> Option<&T> {
        let next_coordinate = self.get_relative_coordinate_right();

        if self.is_point_in_bounds(next_coordinate) {
            return Some(self.get_coordinate(next_coordinate));
        }

        return None
    }
 
    fn position_down(&self) -> Option<&T> {
        let next_coordinate = self.get_relative_coordinate_down();

        if self.is_point_in_bounds(next_coordinate) {
            return Some(self.get_coordinate(next_coordinate));
        }

        return None
    }
 
    fn position_left(&self) -> Option<&T> {
        let next_coordinate = self.get_relative_coordinate_left();

        if self.is_point_in_bounds(next_coordinate) {
            return Some(self.get_coordinate(next_coordinate));
        }

        return None
    }

    fn new_string_from_lines(lines: Vec<String>) -> Grid<String> {
        let mut grid: grid::Grid<String> = grid::grid![];

        for line in lines {
            grid.push_row(split_string_into_characters(line));
        }

        return Grid {
            grid,
            current_position: Point2::new(0, 0)
        };
    }
}

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}
