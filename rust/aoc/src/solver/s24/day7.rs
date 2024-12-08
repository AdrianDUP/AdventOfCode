use crate::solver::solver::{split_string_into_parts, Solver};

pub struct Day7 {
}

impl Solver for Day7 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;
        for line in lines {
            let (result, numbers) = get_data(line);

            let number_of_operators: i64 = numbers.len() as i64 - 1;

            let limit: u32 = u32::pow(2, number_of_operators.try_into().unwrap());

            let mut operations: u32 = 0b0;

            let mut check_answer = 0;

            while operations < limit {
                for (index,number) in numbers.iter().enumerate() {
                    if index as i64 == 0 {
                        check_answer = *number;
                        continue;
                    }

                    let check: u32 = 0b1 << (number_of_operators - index as i64);

                    if operations & check > 0 {
                        check_answer *= number;
                    } else {
                        check_answer += number;
                    }

                    if check_answer > result {
                        break;
                    }
                }

                if check_answer == result {
                    answer += result;
                    break;
                }

                operations += 1;
            }

        }
        return answer;
    }

    fn solution_two(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;

        for line in lines {
            let (result, numbers) = get_data(line);

            let number_of_operations: u32 = numbers.len() as u32 - 1;

            let limit: u32 = u32::pow(4, number_of_operations);

            let mut operations: Vec<u32> = vec![];

            for _ in 0..number_of_operations {
                operations.push(0);
            }

            if operations.len() as u32 != number_of_operations {
                panic!("Operations too many");
            }

            let mut check_answer: i64 = 0;

            for _ in 0..limit {
                for (index, number) in numbers.iter().enumerate() {
                    if index as i64 == 0 {
                        check_answer = *number;
                        continue;
                    }

                    let operation = operations[index-1];

                    if operation == 0 {
                        check_answer *= number;
                    } else if operation == 1 {
                        check_answer += number;
                    } else {
                        check_answer = format!("{check_answer}{number}").parse::<i64>().unwrap();
                    }

                    if check_answer > result {
                        break;
                    }
                }

                if check_answer == result {
                    answer += result;
                    break;
                }

                operations = increment_operations(operations.clone(), 3, operations.len()-1);
            }
        }

        return answer;
    }
}

fn get_data(line: String) -> (i64, Vec<i64>) {
    let elements: Vec<String> = line.split(": ").filter(|x| *x != "").map(String::from).collect();

    let result: i64 = elements[0].parse::<i64>().unwrap();
    let mut numbers: Vec<i64> = vec![];

    let number_strings: Vec<String> = split_string_into_parts(elements[1].clone());

    for number in number_strings {
        numbers.push(number.parse::<i64>().unwrap());
    }

    return (result,numbers);
}

fn increment_operations(mut operations: Vec<u32>, limit: u32, current: usize) -> Vec<u32> {
    let to_increment = operations[current];
    let mut overflow:bool = false;
        
    if to_increment < limit {
        operations[current] = to_increment + 1;
    } else {
        operations[current] = 0;
        overflow = true;
    }

    if current == 0 || !overflow {
        return operations;
    }

    return increment_operations(operations, limit, current - 1);
}
