use std::fmt::Debug;

use crate::solver::solver::Solver;

use recursive::recursive;

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
        let answer: i64 = 0;

        for line in lines {
            let characters: Vec<u64> = line.split("")
                .filter(|x| *x != "")
                .map(|x| String::from(x).parse::<u64>().unwrap())
                .collect();

            let map: Vec<String> = expand_map(characters);

            let fixed_map = move_file_blocks(map.clone(), map.clone());

            dbg!(fixed_map);
            
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

#[recursive]
fn move_file_blocks(map: Vec<String>, map_left: Vec<String>) -> Vec<String> {
    if map_left.is_empty() {
        return map;
    }

    let mut fixed_map: Vec<String> = map.clone();

    let mut is_in_number_block: bool = false;
    let mut current_number: String = ".".to_string();
    let mut number_block_start: usize = 0;
    let mut number_block_end: usize = 0;
    let mut number_block_complete: bool = false;

    let mut is_in_space_block: bool = false;
    let mut space_block_start: usize = 0;
    let mut space_block_end: usize = 0;
    let mut space_block_size: usize = 0;
    let mut number_block_size: usize = 0;

    let mut element: String = map_left.pop().unwrap();

    let mut elements: Vec<String> = vec![];

    if element == ".".to_string() {
    }

    for (index, element) in map.iter().enumerate().rev() {
        dbg!(index, element);
        if element != "." {
            if !is_in_number_block {
                is_in_number_block = true;
                number_block_complete = false;
                number_block_start = index;
                number_block_end = index;
                current_number = element.to_string();
            } else if current_number == *element.to_string() {
                number_block_end = index;
            } else {
                number_block_complete = true;
                is_in_number_block = false;
            }
        } else if is_in_number_block {
            is_in_number_block = false;
            number_block_complete = true;
        }

        if number_block_complete {
            number_block_complete = false;
            for (index2, element2) in fixed_map.iter().enumerate() {
                dbg!(index2, element2);
                if index2 >= index {
                    dbg!(index, index2, "Index hit");
                    break;
                }
                if element2 == "." {
                    if !is_in_space_block {
                        dbg!("Setting space block");
                        is_in_space_block = true;
                        space_block_start = index2;
                        space_block_end = index2;
                    } else {
                        dbg!("Increasing space block");
                        space_block_end = index2;
                    }
                } else if is_in_space_block {
                    dbg!("Ending space block");
                    space_block_size = space_block_end - space_block_start + 1;
                    number_block_size = number_block_start - number_block_end + 1;
                    dbg!(space_block_size, number_block_size);
                    is_in_space_block = false;
                    if space_block_size >= number_block_size {
                        dbg!("Correct size");
                        break;
                    } else {
                        dbg!("Too small");
                    }
                }
            }
            if space_block_size >= number_block_size {
                let mut offset: usize = 0;

                for index3 in number_block_end..=number_block_start {
                    dbg!("Swapping", index3, space_block_start+offset);
                    fixed_map.swap(index3, space_block_start+offset);
                    offset += 1;
                }
                return move_file_blocks(fixed_map, map_left);
            }
        }
    }
    return fixed_map;
}
