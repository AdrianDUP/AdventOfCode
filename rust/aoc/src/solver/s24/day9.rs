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

            let map: Vec<Block> = expand_map2(characters);
            dbg!("Map expanded");
            let fixed_map = move_file_blocks(map.clone(), map.clone(), map.len());
            dbg!("Moved map made");
            let mut final_map: Vec<String> = vec![];
            
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

fn move_file_blocks(mut map_left: Vec<Block>, mut map_added: Vec<Block>, mut current_index: usize) -> Vec<Block> {
    if map_left.is_empty() {
        return map_added;
    }

    let next_element = map_left.pop().unwrap();
    current_index -= 1;

    dbg!(&next_element, current_index);

    if next_element.is_empty {
        return move_file_blocks(map_left, map_added, current_index);
    } else {
        for (index, element) in map_added.iter_mut().enumerate() {
            dbg!(index, &element);
            if index >= current_index {
                break;
            }
            if element.is_empty {
                if element.count < next_element.count {
                    continue;
                } else {
                    if element.count == next_element.count {
                        dbg!("Empty one");
                        map_added[index] = next_element;
                        map_added[current_index].is_empty = true;
                        break;
                    } else {
                        dbg!("Empty two");
                        element.count -= next_element.count;
                        map_added.insert(index, next_element);
                        current_index -= 1;
                        map_added[current_index+1].is_empty = true;
                        break;
                    }
                }
            }
        }
        dbg!(&map_left, &map_added);
        return move_file_blocks(map_left, map_added, current_index);
    }
}
