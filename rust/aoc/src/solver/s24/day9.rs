use std::fmt::Debug;

use crate::solver::solver::Solver;

pub struct Day9 {
}

struct Block {
    is_empty: bool,
    count: u64,
    number: u64,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "is_empty: {}, count: {}, number: {}", self.is_empty, self.count, self.number)
    }
}

impl Solver for Day9 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;

        for line in lines {
            let characters: Vec<u64> = line.split("")
                .filter(|x| *x != "")
                .map(|x| String::from(x).parse::<u64>().unwrap())
                .collect();

            let map: Vec<String> = expand_map(characters);

            let fixed_map = move_elements_to_front(map);

            for (index,element) in fixed_map.iter().enumerate() {
                if element == "." {
                    continue;
                }

                answer += element.parse::<i64>().unwrap() * index as i64;
            }
        }
        return answer;
    }

    fn solution_two(&self, lines: Vec<String>) -> i64 {
        for line in lines {
            let characters: Vec<u64> = line.split("")
                .filter(|x| *x != "")
                .map(|x| String::from(x).parse::<u64>().unwrap())
                .collect();

            let map: Vec<Block> = expand_map2(characters);

            dbg!(map);
        }
        return 0;
    }
}

fn expand_map(details: Vec<u64>) -> Vec<String> {
    let mut current_index: u64 = 0;
    let mut map: Vec<String> = vec![];

    for (i, detail) in details.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..*detail as usize {
                map.push(current_index.to_string());
            }
            current_index += 1;
        } else {
            for _ in 0..*detail as usize {
                map.push(String::from("."));
            }
        }
    }

    return map;
}

fn expand_map2(details: Vec<u64>) -> Vec<Block> {
    let mut map: Vec<Block> = vec![];
    let mut current_index: u64 = 0;

    for (i,detail) in details.iter().enumerate() {
        if i % 2 == 0 {
            let block = Block{
                is_empty: false,
                count: *detail,
                number: current_index,
            };

            map.push(block);
            current_index += 1;
        } else {
            let block = Block{
                is_empty: true,
                count: *detail,
                number: 0,
            };

            map.push(block);
        }
    }

    return map;
}

fn move_elements_to_front(map: Vec<String>) -> Vec<String> {
    let mut position2: usize = 0;
    let mut done: bool = false;

    let mut fixed_map: Vec<String> = vec![];

    for (index, element) in map.iter().enumerate() {
        if done {
            break;
        }
        if element != "." {
            fixed_map.push(element.to_string());
            continue;
        } else {
            for (index2, element2) in map.iter().enumerate().rev() {
                if element2 == "." {
                    continue;
                } else if position2 != 0 && index2 >= position2 {
                    continue;
                } else if index2 < index {
                    done = true;
                    break;
                }

                position2 = index2;
                fixed_map.push(element2.to_string());
                break;
            }
        }
    }

    return fixed_map;
}

fn move_file_blocks(mut map_left: Vec<Block>, map_added: Vec<Block>) -> Vec<String> {
    if map_left.is_empty() {
        return map_added;
    }

    let next_element = map_left.pop().unwrap();

    if next_element.is_empty {
        return move_file_blocks(map_left, map_added);
    } else {
        let mut moved: bool = false;

        while !moved {
            let start = map_left.remove(0);

            if !start.is_empty {
                map_added.push(start);
            } else if start.count >= next_element.count {
            }
        }
    }
}

fn is_sorted(map: Vec<String>) -> bool {
    let mut space_found: bool = false;

    for element in map {
        if element != "." && space_found {
            return false;
        } else if element == "." {
            space_found = true;
        }
    }

    return true;
}
