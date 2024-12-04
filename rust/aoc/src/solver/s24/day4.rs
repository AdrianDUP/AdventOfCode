use diagonal::{diagonal_pos_neg, diagonal_pos_pos};

use crate::solver::solver::Solver;

pub struct Day4 {
}

impl Solver for Day4 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;

        let rows_and_columns = get_rows_and_columns(lines);

        let columns_as_rows = get_columns_as_rows(rows_and_columns.clone());
        let diagonal = get_diagonals(rows_and_columns.clone());
        let diagonal_pos = get_diagonals_pos(rows_and_columns.clone());

        for element in rows_and_columns {
            answer += find_count_of_xmas(element);
        }
        for element in columns_as_rows {
            answer += find_count_of_xmas(element);
        }
        for element in diagonal {
            answer += find_count_of_xmas(element);
        }
        for element in diagonal_pos {
            answer += find_count_of_xmas(element);
        }

        return answer;
    }

    fn solution_two(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;

        let rows_and_columns = get_rows_and_columns(lines);

        for (index,row) in rows_and_columns.iter().enumerate() {
            if index == 0 || index == rows_and_columns.len() - 1 {
                continue;
            }

            for (cindex, char) in row.iter().enumerate() {
                if char != "A" {
                    continue;
                }

                if cindex == 0 || cindex == row.len()-1 {
                    continue;
                }

                if rows_and_columns[index-1][cindex-1] == "M" && rows_and_columns[index+1][cindex+1] == "S" || rows_and_columns[index-1][cindex-1] == "S" && rows_and_columns[index+1][cindex+1] == "M" {
                    if rows_and_columns[index+1][cindex-1] == "M" && rows_and_columns[index-1][cindex+1] == "S" || rows_and_columns[index+1][cindex-1] == "S" && rows_and_columns[index-1][cindex+1] == "M" {
                        answer += 1;
                    }
                }
            }
        }

        return answer;
    }
}

fn get_rows_and_columns(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut rows_and_columns: Vec<Vec<String>> = vec![];

    for line in lines {
        let columns = line.split("").map(String::from).collect();
        rows_and_columns.push(columns);
    }

    return rows_and_columns;
}

fn get_columns_as_rows(rows_and_columns: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut rows_as_columns: Vec<Vec<String>> = vec![];

    for column in 0..rows_and_columns[0].len() {
        let mut new_column: Vec<String> = vec![];
        for row in 0..rows_and_columns.len() {
            new_column.push(rows_and_columns[row][column].clone());
        }
        rows_as_columns.push(new_column);
    }

    return rows_as_columns;
}

fn get_diagonals_pos(rows_and_columns: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let copy = rows_and_columns.clone();
    let diagnals =  diagonal_pos_pos(&copy);

    let mut value: Vec<Vec<String>> = vec![];

    for diag in diagnals {
        let mut new_diag: Vec<String> = vec![];
        for element in diag {
            new_diag.push(element.clone());
        }
        value.push(new_diag);
    }

    return value;
}

fn get_diagonals(rows_and_columns: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let copy = rows_and_columns.clone();
    let diagnals =  diagonal_pos_neg(&copy);

    let mut value: Vec<Vec<String>> = vec![];

    for diag in diagnals {
        let mut new_diag: Vec<String> = vec![];
        for element in diag {
            new_diag.push(element.clone());
        }
        value.push(new_diag);
    }

    return value;
    //
    // let column_count_index = rows_and_columns[0].len().saturating_sub(1);
    // dbg!(column_count_index);
    // let row_count_index = rows_and_columns.len().saturating_sub(1);
    // dbg!(row_count_index);
    //
    // let mut row_offset: i32 = 0;
    //
    // for column_index in 0..column_count_index {
    //     let max_length = column_count_index.saturating_sub(column_index.saturating_add(1));
    //     let mut diagonal: Vec<String> = vec![];
    //
    //     for item_index in column_index..max_length {
    //         let offset_index = item_index.saturating_sub(row_offset.try_into().unwrap());
    //         diagonal.push(rows_and_columns[offset_index][item_index].clone());
    //     }
    //
    //     diagonals.push(diagonal);
    //     row_offset += 1;
    // }
    //
    // let mut column_offset = 1;
    //
    // for row_index in 1..row_count_index {
    //     let max_length = row_count_index - row_index;
    //     let mut diagonal: Vec<String> = vec![];
    //
    //     for item_index in row_index..max_length {
    //         diagonal.push(rows_and_columns[item_index][item_index.saturating_sub(column_offset)].clone());
    //     }
    //
    //     diagonals.push(diagonal);
    //     column_offset += 1;
    // }
    //
}

fn find_count_of_xmas(line: Vec<String>) -> i64 {
    let mut tracking_forward = false;
    let mut forward_index = 0;
    let mut tracking_backward = false;
    let mut backward_index = 0;
    let mut count: i64 = 0;

    dbg!(&line);

    if line.len() < 4 {
        return count;
    }

    for char in line {
        if char == "S" {
            tracking_backward = true;
            backward_index = 0;

            if tracking_forward && forward_index == 2 {
                count += 1;
                tracking_forward = false;
            } else {
                tracking_forward = false;
                forward_index = 0;
            }
        } else if char == "X" {
            tracking_forward = true;
            forward_index = 0;

            if tracking_backward && backward_index == 2 {
                count += 1;
                tracking_backward = false;
            } else {
                tracking_backward = false;
                backward_index = 0;
            }
        } else if char == "M" {
            if tracking_backward {
                if backward_index == 1 {
                    backward_index += 1;
                } else {
                    tracking_backward = false;
                    backward_index = 0;
                }
            }
            if tracking_forward && forward_index == 0 {
                forward_index += 1;
            } else {
                tracking_forward = false;
                forward_index = 0;
            }
        } else {
            if tracking_forward {
                if forward_index == 1 {
                    forward_index += 1;
                } else {
                    tracking_forward = false;
                    forward_index = 0;
                }
            }

            if tracking_backward && backward_index == 0 {
                backward_index += 1;
            } else {
                tracking_backward = false;
                backward_index = 0;
            }
        }
    }

    dbg!(count);

    return count;
}
