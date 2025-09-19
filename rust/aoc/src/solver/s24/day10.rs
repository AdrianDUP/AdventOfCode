use std::collections::{HashSet, VecDeque};

use crate::solver::solver::Solver;

pub struct Day10 {}

impl Solver for Day10 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        if lines.is_empty() { return 0; }
        let grid = parse_grid(&lines);
        let h = grid.len();
        let w = grid[0].len();

        // collect all trailheads (cells with height 0)
        let mut total: i64 = 0;
        for r in 0..h {
            for c in 0..w {
                if grid[r][c] == 0 {
                    total += score_trailhead(&grid, r, c) as i64;
                }
            }
        }
        total
    }

    fn solution_two(&self, lines: Vec<String>) -> i64 {
        if lines.is_empty() { return 0; }
        let grid = parse_grid(&lines);
        let h = grid.len();
        let w = grid[0].len();

        // Precompute number of distinct paths from each cell to any 9 using DP by height
        let paths = compute_paths(&grid);

        let mut total: i64 = 0;
        for r in 0..h {
            for c in 0..w {
                if grid[r][c] == 0 {
                    total += paths[r][c] as i64;
                }
            }
        }
        total
    }
}

fn parse_grid(lines: &Vec<String>) -> Vec<Vec<u8>> {
    lines.iter()
        .map(|line| line.chars().map(|ch| ch as u8 - b'0').collect())
        .collect()
}

fn score_trailhead(grid: &Vec<Vec<u8>>, sr: usize, sc: usize) -> usize {
    let h = grid.len();
    let w = grid[0].len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((sr, sc));
    visited.insert((sr, sc));
    let mut nines: HashSet<(usize, usize)> = HashSet::new();

    while let Some((r, c)) = q.pop_front() {
        let height = grid[r][c];
        if height == 9 {
            nines.insert((r, c));
            continue;
        }
        let next_h = height + 1;
        // explore neighbors up, down, left, right if height increases by exactly 1
        if r > 0 && grid[r - 1][c] == next_h && visited.insert((r - 1, c)) {
            q.push_back((r - 1, c));
        }
        if r + 1 < h && grid[r + 1][c] == next_h && visited.insert((r + 1, c)) {
            q.push_back((r + 1, c));
        }
        if c > 0 && grid[r][c - 1] == next_h && visited.insert((r, c - 1)) {
            q.push_back((r, c - 1));
        }
        if c + 1 < w && grid[r][c + 1] == next_h && visited.insert((r, c + 1)) {
            q.push_back((r, c + 1));
        }
    }

    nines.len()
}

fn compute_paths(grid: &Vec<Vec<u8>>) -> Vec<Vec<u64>> {
    let h = grid.len();
    let w = grid[0].len();
    let mut dp: Vec<Vec<u64>> = vec![vec![0; w]; h];

    // Process cells in decreasing height order
    for height in (0..=9u8).rev() {
        for r in 0..h {
            for c in 0..w {
                if grid[r][c] != height { continue; }
                if height == 9 {
                    dp[r][c] = 1; // each 9 is an endpoint contributing one trail
                } else {
                    let nh = height + 1;
                    let mut sum: u64 = 0;
                    if r > 0 && grid[r - 1][c] == nh { sum += dp[r - 1][c]; }
                    if r + 1 < h && grid[r + 1][c] == nh { sum += dp[r + 1][c]; }
                    if c > 0 && grid[r][c - 1] == nh { sum += dp[r][c - 1]; }
                    if c + 1 < w && grid[r][c + 1] == nh { sum += dp[r][c + 1]; }
                    dp[r][c] = sum;
                }
            }
        }
    }

    dp
}
