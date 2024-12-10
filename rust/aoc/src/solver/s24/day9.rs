use std::fmt::Debug;

use crate::solver::solver::Solver;

pub struct Day9 {
}

#[derive(Clone)]
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
        let mut answer: i64 = 0;
        dbg!("Starting");

        for line in lines {
            let characters: Vec<u64> = line.split("")
                .filter(|x| *x != "")
                .map(|x| String::from(x).parse::<u64>().unwrap())
                .collect();
            dbg!("Characters collected");

            let map: Vec<String> = expand_map(characters);
            
            for block in fixed_map {
                let char: String;

                if block.is_empty {
                    char = ".".to_string();
                } else {
                    char = block.number.to_string();
                }
                
                for _ in 0..block.count {
                    final_map.push(char.clone());
                }
            }
            dbg!("Final map made", &final_map);
            
            for (index, number) in final_map.iter().enumerate() {
                if number == "." {
                    continue;
                }

                answer += index as i64 * number.parse::<i64>().unwrap();
            }
        }
        return answer;
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

fn move_file_blocks(map: Vec<String>) -> Vec<String> {
    let mut fixed_map: Vec<String> = map.clone();

    let mut is_in_number_block: bool = false;
    let mut current_number: String = ".".to_string();
    let mut number_block_start: usize = 0;
    let mut number_block_end: usize = 0;

    let mut is_in_space_block: bool = false;
    let mut space_block_start: usize = 0;
    let mut space_block_end: usize = 0;

    for (index, element) in map.iter().enumerate().rev() {
        if element != "." {
            if !is_in_number_block {
                is_in_number_block = true;
                number_block_start = index;
                number_block_end = index;
                current_number = element.to_string();
                continue;
            } else if current_number != *element {
                number_block_end = index;
                continue;
            } else {
                number_block_end = index;
                continue;
            }
        } else {
            if is_in_number_block {
                is_in_number_block = false;

                for (index1, element2) in map.iter().enumerate() {
                    if index1 >= index {
                        break;
                    }

                    if element2 == "." {
                        if !is_in_space_block {
                            is_in_space_block = true;
                            space_block_start = index1;
                            space_block_end = index1;
                        } else {
                            space_block_end = index1;
                        }
                    } else {
                        if is_in_space_block {
                            is_in_space_block = false;

                            let space_block_size = space_block_end - space_block_start;
                            let number_block_size = number_block_start - number_block_end;
                            if space_block_size >= number_block_size {
                                let offset: usize = 0;
                                for index in number_block_start..=number_block_end {
                                    fixed_map.swap(index, space_block_start+offset);
                                    return move_file_blocks(fixed_map);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return fixed_map;
}
