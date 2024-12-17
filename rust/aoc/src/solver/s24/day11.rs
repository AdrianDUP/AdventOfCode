use crate::solver::solver::{split_string_into_parts, Solver};

pub struct Day11 {
}

impl Solver for Day11 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;
        for line in lines {
            let mut numbers: Vec<i64> = split_string_into_parts(line).into_iter().map(|x| x.parse::<i64>().unwrap()).collect();

            for _ in 0..25 {
                numbers = blink(numbers.clone());
            }

            answer += numbers.len() as i64;
        }
        return answer;
    }

    fn solution_two(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;
        for line in lines {
            let mut numbers: Vec<i64> = split_string_into_parts(line).into_iter().map(|x| x.parse::<i64>().unwrap()).collect();

            for i in 0..75 {
                println!("Blinked {i} times");
                numbers = blink(numbers.clone());
            }

            answer += numbers.len() as i64;
        }
        return answer;
    }
}

fn blink(numbers: Vec<i64>) -> Vec<i64> {
    let mut new_array: Vec<i64> = vec![];

    for number in numbers {
        if number == 0 {
            new_array.push(handle_stone_zero());
        } else if is_even_number_of_digits(number) {
            let (first,last) = handle_even_stone(number);
            new_array.push(first);
            new_array.push(last);
        } else {
            new_array.push(handle_default_case(number));
        }
    }
    return new_array;
}

fn handle_stone_zero() -> i64 {
    return 1;
}

fn is_even_number_of_digits(number: i64) -> bool {
    return number.to_string().len() % 2 == 0;
}

fn handle_even_stone(number: i64) -> (i64, i64) {
    let number_string: String = number.to_string();
    let half_way: usize = number_string.len() / 2;

    let (left, right) = number_string.split_at(half_way);
    return (left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap());
}

fn handle_default_case(number: i64) -> i64 {
    return number * 2024;
}
