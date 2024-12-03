use crate::solver::solver::Solver;

pub struct Day2 {
}

impl Solver for Day2 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;

        for line in lines {
            let numbers = get_numbers(line);
            
            if is_safe_report(numbers) {
                answer += 1;
            }
        }

        return answer;
    }

    fn solution_two(&self, lines: Vec<String>) -> i64 {
        for line in lines {
            todo!();
        }

        return 0;
    }
}

fn get_numbers(line: String) -> Vec<i64> {
    return line.split_whitespace()
        .map(|e| e.parse::<i64>().unwrap())
        .collect();
}

fn is_safe_report(numbers: Vec<i64>) -> bool {
    let mut previous_number: i64 = 0;
    let mut is_ascending: bool = false;

    for (index, number) in numbers.iter().enumerate() {
        if index == 0 {
            previous_number = *number;
            continue;
        }

        if index == 1 {
            is_ascending = *number > previous_number;
        }

        if !is_safe_sequence(previous_number, *number, is_ascending) {
            return false;
        }

        previous_number = *number;
    }

    return true;
}

fn is_safe_sequence(previous_number: i64, current_number: i64, is_ascending: bool) -> bool {
    if previous_number == current_number {
        return false;
    }

    if is_ascending && previous_number > current_number || !is_ascending && previous_number < current_number {
        return false;
    }

    let difference: i64 = previous_number - current_number;
    
    if difference.abs() > 3 {
        return false;
    }

    return true;
}
