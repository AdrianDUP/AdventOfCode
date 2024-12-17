use std::intrinsics::floorf64;

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
}

impl Computer {
    fn perform_next_operation(&mut self) {
        self.current_instruction = self.get_pointer_value();
        self.advance_pointer();
        self.current_operand = self.get_pointer_value();
    }

    fn execute_instruction(&self, instruction: i64, operand: i64) {

    }

    fn perform_div_instruction(&self, operand: i64, numerator: i64) -> i64 {
        let answer = numerator / 2i64.pow(self.get_combo_operand(operand));
        return math::round::floor(answer);
    }

    fn perform_adv_opcode(&mut self, operand: i64) {
        self.register_a = self.perform_div_instruction(operand, self.register_a);
    }
    
    fn perform_bxl_opcode(&mut self, operand: i64) {
        self.register_b = self.register_b ^ operand;
    }
    
    fn perform_bst_opcode(&mut self, operand: i64) {
    }

    fn perform_jnz_instruction() {
    }

    fn perform_bxc_opcode() {
    }

    fn perform_out_opcode() {
    }

    fn perform_bdv_opcode(&self, operand: i64) {
        self.register_b = self.perform_div_instruction(operand, self.register_a);
    }

    fn perform_cdv_opcode(&self, operand: i64) {
        self.register_c = self.perform_div_instruction(operand, self.register_a);
    }

    fn advance_pointer(&mut self) {
        self.pointer += 1;
    }

    fn get_pointer_value(&self) {
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
        return 0;
    }

    fn solution_two(&self, lines: Vec<String>) -> i64 {
        return 0;
    }
}
