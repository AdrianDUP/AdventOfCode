use std::{sync::Mutex, thread};

use threadpool::ThreadPool;

use crate::solver::solver::Solver;

pub struct Day17 {
}

struct Computer {
    instructions: Vec<i64>,
    pointer: usize,
    current_instruction: i64,
    current_operand: i64,
    register_a: i64,
    register_b: i64,
    register_c: i64,
    output: Vec<i64>,
}

impl Computer {
    fn run(&mut self) {
        while self.pointer < self.instructions.len() {
            self.perform_next_operation();
        }
    }

    fn perform_next_operation(&mut self) {
        self.current_instruction = self.get_pointer_value();
        self.advance_pointer();
        if self.pointer >= self.instructions.len() {
            return;
        }
        self.current_operand = self.get_pointer_value();
        self.advance_pointer();

        self.execute_instruction(self.current_instruction, self.current_operand);
        if self.pointer >= self.instructions.len() {
            return;
        }
    }

    fn execute_instruction(&mut self, instruction: i64, operand: i64) {
        match instruction {
            0 => self.perform_adv_opcode(operand),
            1 => self.perform_bxl_opcode(operand),
            2 => self.perform_bst_opcode(operand),
            3 => self.perform_jnz_instruction(),
            4 => self.perform_bxc_opcode(),
            5 => self.perform_out_opcode(),
            6 => self.perform_bdv_opcode(operand),
            7 => self.perform_cdv_opcode(operand),
            _ => panic!("Invalid instruction"),
        }
    }

    fn perform_div_instruction(&self, operand: i64, numerator: i64) -> i64 {
        return TryInto::<i64>::try_into(numerator / 2i64.pow(self.get_combo_operand(operand).try_into().unwrap())).unwrap();
    }

    fn perform_adv_opcode(&mut self, operand: i64) {
        self.register_a = self.perform_div_instruction(operand, self.register_a);
    }
    
    fn perform_bxl_opcode(&mut self, operand: i64) {
        self.register_b = self.register_b ^ operand;
    }
    
    fn perform_bst_opcode(&mut self, operand: i64) {
        self.register_b = self.get_combo_operand(operand) % 8;
    }

    fn perform_jnz_instruction(&mut self) {
        if self.register_a == 0 {
            return;
        }

        self.pointer = self.current_operand as usize;
    }

    fn perform_bxc_opcode(&mut self) {
        self.register_b = self.register_b ^ self.register_c;
    }

    fn perform_out_opcode(&mut self) {
        self.output.push(self.get_combo_operand(self.current_operand)%8);
    }

    fn perform_bdv_opcode(&mut self, operand: i64) {
        self.register_b = self.perform_div_instruction(operand, self.register_a);
    }

    fn perform_cdv_opcode(&mut self, operand: i64) {
        self.register_c = self.perform_div_instruction(operand, self.register_a);
    }

    fn advance_pointer(&mut self) {
        self.pointer += 1;
    }

    fn get_pointer_value(&self) -> i64 {
        return self.instructions[self.pointer];
    }
    
    fn get_combo_operand(&self, operand: i64) -> i64 {
        return match operand {
            0|1|2|3 => operand,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => 0
        };
    }
}

impl Solver for Day17 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let mut register_a: i64 = 0;
        let mut register_b: i64 = 0;
        let mut register_c: i64 = 0;
        let mut program_instructions: Vec<i64> = vec![];

        for (index, line) in lines.iter().enumerate() {
            if index == 0 {
                register_a = extract_register_value(line.to_string());
            } else if index == 1 {
                register_b = extract_register_value(line.to_string());
            } else if index == 2 {
                register_c = extract_register_value(line.to_string());
            }

            if line == "" {
                continue;
            }

            program_instructions = load_instructions(line.to_string());
        }

        let mut computer = Computer{
            register_a,
            register_b,
            register_c,
            instructions: program_instructions,
            pointer: 0,
            current_instruction: 0,
            current_operand: 0,
            output: vec![],
        };

        computer.run();

        return 0;
    }

    fn solution_two(&self, lines: Vec<String>) -> i64 {
        // let mut handles = vec![];
        let thread_pool = ThreadPool::new(20);
        let factor: usize = 100000000000;

        for i in 0..20 {
            let values = lines.clone();
            let factor_copy = factor;
            let _ = thread_pool.execute(move || {
                let multiplier: usize = i;
                println!("Multiplier for thread {i} is {multiplier}");
                let limit: usize = multiplier * factor_copy;

                let mut register_a: i64 = 0;
                let mut register_b: i64 = 0;
                let mut register_c: i64 = 0;
                let mut program_instructions: Vec<i64> = vec![];

                for (index, line) in values.iter().enumerate() {
                    if index == 0 {
                        register_a = extract_register_value(line.to_string());
                    } else if index == 1 {
                        register_b = extract_register_value(line.to_string());
                    } else if index == 2 {
                        register_c = extract_register_value(line.to_string());
                    }

                    if line == "" {
                        continue;
                    }

                    program_instructions = load_instructions(line.to_string());
                }

                let mut computer = Computer{
                    register_a,
                    register_b,
                    register_c,
                    instructions: program_instructions.clone(),
                    pointer: 0,
                    current_instruction: 0,
                    current_operand: 0,
                    output: vec![],
                };

                for j in limit..limit+factor_copy {
                    computer.register_a = j as i64;
                    computer.register_b = 0;
                    computer.register_c = 0;
                    computer.output = vec![];
                    computer.pointer = 0;
                    // if j % 1000000 == 0 {
                    //     println!("Starting new loop with {j}");
                    // }
                    computer.run();

                    if computer.output == computer.instructions {
                        println!("Found answer {j}");
                        break;
                    }
                }
            });

            // handle.join();
        }

        thread_pool.join();

        // for handle in handles {
        //     handle.join().unwrap();
        // }

        // let expected_output = program_instructions.clone();
        // computer.output = vec![];
        //
        // let counter: Mutex<usize> = Mutex::new(0);
        //
        // for _ in 0..10 {
        //     let handle = thread::spawn(move || {
        //         let mut limit = counter.lock().unwrap();
        //         *limit += 1000000000;
        //
        //         for i in limit-1000000000..limit {
        //             if i as i64 == register_a {
        //                 continue;
        //             }
        //             computer.register_a = i as i64;
        //             computer.register_b = 0;
        //             computer.register_c = 0;
        //             computer.output = vec![];
        //             computer.pointer = 0;
        //             if i % 1000000 == 0 {
        //                 println!("Starting new loop with {i}");
        //             }
        //             computer.run();
        //
        //             if computer.output == expected_output {
        //                 answer = i as i64;
        //                 break;
        //             }
        //         }
        //     });
        // }

        // for i in 1000000000..1000000000000 {
        //     if i == register_a {
        //         continue;
        //     }
        //     computer.register_a = i as i64;
        //     computer.register_b = 0;
        //     computer.register_c = 0;
        //     computer.output = vec![];
        //     computer.pointer = 0;
        //     if i % 1000000 == 0 {
        //         println!("Starting new loop with {i}");
        //     }
        //     computer.run();
        //
        //     if computer.output == expected_output {
        //         answer = i as i64;
        //         break;
        //     }
        // }

        // dbg!(computer.output);

        return 0;
    }
}

fn extract_register_value(line: String) -> i64 {
    return line.split(": ")
        .map(String::from)
        .collect::<Vec<String>>()
        .pop()
        .unwrap()
        .parse::<i64>()
        .unwrap();
}

fn load_instructions(line: String) -> Vec<i64> {
    let parts = line.split(": ")
        .map(String::from)
        .collect::<Vec<String>>()
        .pop()
        .unwrap();

    return parts.split(",")
        .map(|e| e.to_string().parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}
