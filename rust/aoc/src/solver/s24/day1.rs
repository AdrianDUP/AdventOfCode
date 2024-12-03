use std::collections::HashMap;

use crate::solver::solver::Solver;

pub struct Day1 {
}

impl Solver for Day1 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;
        
        let mut left_numbers: Vec<i64> = vec![];
        let mut right_numbers: Vec<i64> = vec![];

        for line in lines {
            let (left_number, right_number) = get_numbers_from_line(line);
            
            left_numbers.push(left_number);
            right_numbers.push(right_number);
        }

        for (index,number) in left_numbers.iter().enumerate() {
            let distance = number - right_numbers[index];

            answer += distance.abs();
        }

        return answer;
    }

    fn solution_two(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;

        let mut left_numbers: Vec<i64> = vec![];
        let mut right_numbers: Vec<i64> = vec![];

        for line in lines {
            let (left_number, right_number) = get_numbers_from_line(line);

            left_numbers.push(left_number);
            right_numbers.push(right_number);
        }

        let occurrences = count_occurrences(right_numbers);

        for number in left_numbers {
            if occurrences.contains_key(&number) {
                answer += number * occurrences[&number];
            }
        }

        return answer;
    }
}

fn get_numbers_from_line(line: String) -> (i64, i64) {
    let regx = regex::Regex::new(r"[0-9]+").unwrap();
    let numbers: Vec<&str> = regx.find_iter(&line).map(|e| e.as_str()).collect();

    if numbers.len() != 2 {
        return (0,0);
    } else {
        return (numbers[0].parse::<i64>().unwrap(), numbers[1].parse::<i64>().unwrap());
    }
}

fn count_occurrences(numbers: Vec<i64>) -> HashMap<i64, i64> {
    let mut the_hash: HashMap<i64, i64> = HashMap::new();

    for number in numbers {
        the_hash.entry(number).and_modify(|k| *k += 1).or_insert(1);
    }

    return the_hash;
}
