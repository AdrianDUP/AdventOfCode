use std::{cmp::Ordering, collections::HashMap};

use crate::solver::solver::{find_all_by_regex, Solver};

pub struct Day5 {
}

impl Solver for Day5 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;
        let mut has_all_rules = false;
        let mut rules: HashMap<i64, Vec<i64>> = HashMap::new();
        let mut manuals: Vec<Vec<i64>> = vec![];

        for line in lines {
            if line == "" {
                has_all_rules = true;
                continue;
            }
            if !has_all_rules {
                let (left, right) = get_rule_numbers(line);

                match rules.get_mut(&left) {
                    Some(rule) => {
                        rule.push(right);
                    },
                    None => {
                        rules.insert(left, vec![right]);
                    }
                }
            } else {
                manuals.push(get_manual_numbers(line));
            }
        }

        for manual in manuals {
            if is_correct_update(rules.clone(), manual.clone()) {
                let middle = manual.len() / 2;
                answer += manual[middle];
            }
        }
        return answer;
    }

    fn solution_two(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;
        let (rules,manuals) = get_rules_and_manuals(lines);
        let mut incorrect_manuals: Vec<Vec<i64>> = vec![];

        for manual in manuals {
            if !is_correct_update(rules.clone(), manual.clone()) {
                incorrect_manuals.push(manual);
            }
        }

        for incorrect_manual in incorrect_manuals {
            let fixed = fix_manual(rules.clone(), incorrect_manual);
            
            let middle = fixed.len() / 2;
            answer += fixed[middle];
        }

        return answer;
    }
}

fn get_rules_and_manuals(lines: Vec<String>) -> (HashMap<i64, Vec<i64>>, Vec<Vec<i64>>) {
    let mut has_all_rules = false;
    let mut rules: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut manuals: Vec<Vec<i64>> = vec![];

    for line in lines {
        if line == "" {
            has_all_rules = true;
            continue;
        }
        if !has_all_rules {
            let (left, right) = get_rule_numbers(line);

            match rules.get_mut(&left) {
                Some(rule) => {
                    rule.push(right);
                },
                None => {
                    rules.insert(left, vec![right]);
                }
            }
        } else {
            manuals.push(get_manual_numbers(line));
        }
    }

    return (rules,manuals);
}

fn is_correct_update(rules: HashMap<i64, Vec<i64>>, manual: Vec<i64>) -> bool {
    for (index,number) in manual.iter().enumerate() {
        if index == manual.len()-1 {
            continue;
        }

        let next_number = manual[index+1];

        if let Some(rule) = rules.get(number) {
            if rule.contains(&next_number) {
                continue;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    return true;
}

fn get_rule_numbers(line: String) -> (i64, i64) {
    let number_strings = find_all_by_regex(line, r"[0-9]{2}");

    if number_strings.len() != 2 {
        panic!("Invalid Rules");
    }

    return (number_strings[0].parse::<i64>().unwrap(), number_strings[1].parse::<i64>().unwrap());
}

fn get_manual_numbers(line: String) -> Vec<i64> {
    let number_strings = find_all_by_regex(line, r"[0-9]{2}");
    let mut numbers: Vec<i64> = vec![];

    for number in number_strings {
        numbers.push(number.parse::<i64>().unwrap());
    }

    return numbers;
}

fn fix_manual(rules: HashMap<i64, Vec<i64>>, mut manual: Vec<i64>) -> Vec<i64> {
    manual.sort_by(|a, b| { 
        if !rules.contains_key(a){ 
            Ordering::Greater 
        } else if rules.get(a).unwrap().contains(b) { 
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });

    return manual;
}
