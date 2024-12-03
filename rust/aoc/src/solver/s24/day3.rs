use crate::solver::solver::Solver;

pub struct Day3 {
}

impl Solver for Day3 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;

        for line in lines {
            let multipliers = get_multipliers(line);

            for multiplier in multipliers {
                answer += multiply(multiplier);
            }
        }

        return answer;
    }

    fn solution_two(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;
        let mut enable: bool = true;

        for line in lines {
            let instructions = get_all_instructions(line);

            for instruction in instructions {
                if instruction == "do()" {
                    enable = true;
                    continue;
                } else if instruction == "don't()" {
                    enable = false;
                    continue;
                } else if enable {
                    answer += multiply(instruction);
                }
            }
        }
        
        return answer;
    }
}

fn get_multipliers(line: String) -> Vec<String> {
    let regx = regex::Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    return regx.find_iter(&line).map(|e| String::from(e.as_str())).collect();
}

fn get_all_instructions(line: String) -> Vec<String> {
    let regx = regex::Regex::new(r"do\(\)|mul\(\d{1,3},\d{1,3}\)|don't\(\)").unwrap();

    return regx.find_iter(&line).map(|e| String::from(e.as_str())).collect();
}

fn multiply(multiplier: String) -> i64 {
    dbg!(&multiplier);
    let regx = regex::Regex::new(r"\d+").unwrap();

    let numbers: Vec<i64> = regx.find_iter(&multiplier).map(|e| e.as_str().parse::<i64>().unwrap()).collect();

    if numbers.len() != 2 {
        return 0;
    }

    return numbers[0] * numbers[1];
}
