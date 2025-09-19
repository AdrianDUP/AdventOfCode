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
        if lines.is_empty() { return 0; }
        let line = &lines[0];

        let (mut blocks, mut file_starts, file_lens) = parse_disk(line);
        let mut free_segs = build_free_segments(&blocks);

        // move files in decreasing id order
        let mut fid: i64 = (file_starts.len() as i64) - 1;
        while fid >= 0 {
            let L = file_lens[fid as usize];
            if L == 0 { fid -= 1; continue; }
            let start = file_starts[fid as usize];
            // find leftmost fit strictly before current start
            if let Some(idx) = find_leftmost_fit(&free_segs, start, L) {
                let seg = free_segs[idx];
                let new_start = seg.start;
                // fill target positions with fid
                for k in 0..L { blocks[new_start + k] = fid; }
                // update or remove the used free segment prefix
                if seg.length == L {
                    free_segs.remove(idx);
                } else {
                    free_segs[idx].start += L;
                    free_segs[idx].length -= L;
                }
                // free the old file location
                for k in 0..L { blocks[start + k] = -1; }
                // insert freed segment and possibly merge
                insert_free_segment(&mut free_segs, start, L);
                // update file start
                file_starts[fid as usize] = new_start;
            }
            fid -= 1;
        }

        // compute checksum
        let mut checksum: i64 = 0;
        for (i, v) in blocks.iter().enumerate() {
            if *v >= 0 { checksum += (i as i64) * (*v as i64); }
        }
        checksum
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
fn move_file_blocks(map: Vec<String>, mut map_left: Vec<String>) -> Vec<String> {
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


#[derive(Copy, Clone, Debug)]
struct FreeSeg { start: usize, length: usize }

fn parse_disk(line: &str) -> (Vec<i64>, Vec<usize>, Vec<usize>) {
    // collect run lengths (digits)
    let mut run_lens: Vec<usize> = Vec::with_capacity(line.len());
    for ch in line.chars() {
        if let Some(d) = ch.to_digit(10) { run_lens.push(d as usize); }
    }
    let mut total = 0usize;
    let mut file_count = 0usize;
    for (i, v) in run_lens.iter().enumerate() { total += *v; if i % 2 == 0 { file_count += 1; } }
    let mut blocks: Vec<i64> = vec![-1; total];
    let mut file_starts: Vec<usize> = vec![0; file_count];
    let mut file_lens: Vec<usize> = vec![0; file_count];

    let mut pos = 0usize;
    let mut file_id = 0i64;
    for (i, &v) in run_lens.iter().enumerate() {
        if v == 0 { continue; }
        if i % 2 == 0 {
            file_starts[file_id as usize] = pos;
            file_lens[file_id as usize] = v;
            for k in 0..v { blocks[pos + k] = file_id; }
            pos += v;
            file_id += 1;
        } else {
            // free already set to -1
            pos += v;
        }
    }
    (blocks, file_starts, file_lens)
}

fn build_free_segments(blocks: &Vec<i64>) -> Vec<FreeSeg> {
    let n = blocks.len();
    let mut segs: Vec<FreeSeg> = Vec::new();
    let mut i = 0usize;
    while i < n {
        if blocks[i] == -1 {
            let mut j = i + 1;
            while j < n && blocks[j] == -1 { j += 1; }
            segs.push(FreeSeg{start: i, length: j - i});
            i = j;
        } else { i += 1; }
    }
    segs
}

fn find_leftmost_fit(segs: &Vec<FreeSeg>, limit: usize, need: usize) -> Option<usize> {
    for (idx, s) in segs.iter().enumerate() {
        if s.start >= limit { break; }
        if s.length >= need { return Some(idx); }
    }
    None
}

fn insert_free_segment(segs: &mut Vec<FreeSeg>, start: usize, length: usize) {
    // find insertion index to keep sorted by start
    let mut i = 0usize;
    while i < segs.len() && segs[i].start < start { i += 1; }
    segs.insert(i, FreeSeg{start, length});
    // merge with previous if adjacent
    if i > 0 {
        let prev = segs[i-1];
        let curr = segs[i];
        if prev.start + prev.length == curr.start {
            segs[i-1].length += curr.length;
            segs.remove(i);
            i -= 1;
        }
    }
    // merge with next if adjacent
    if i + 1 < segs.len() {
        let curr = segs[i];
        let next = segs[i+1];
        if curr.start + curr.length == next.start {
            segs[i].length += next.length;
            segs.remove(i+1);
        }
    }
}
