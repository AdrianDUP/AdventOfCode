use crate::solver::solver::Solver;

pub struct Day20 {
}

#[derive(Debug, Copy, Clone)]
struct SecretNumber {
    number: i64,
}

impl SecretNumber {
    fn mix(&mut self, number_to_mix: i64) {
        self.number = self.number ^ number_to_mix;
    }

    fn prune(&mut self) {
        self.number = self.number % 16777216;
    }

    fn calculate_nth_iteration(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.iterate();
        }
    }

    fn iterate(&mut self) {
        self.step_one();
        self.step_two();
        self.step_three();
    }

    fn step_one(&mut self) {
        let calculation_one: i64 = self.number * 64;
        self.mix(calculation_one);
        self.prune();
    }

    fn step_two(&mut self) {
        let calculation: i64 = self.number / 32;
        self.mix(calculation);
        self.prune();
    }

    fn step_three(&mut self) {
        let calculation: i64 = self.number * 2048;
        self.mix(calculation);
        self.prune();
    }
}

impl Solver for Day20 {
    fn solution_one(&self, lines: Vec<String>) -> i64 {
        let mut answer: i64 = 0;

        let numbers: Vec<SecretNumber> = lines.into_iter()
            .map(|e| SecretNumber{number: e.parse::<i64>().unwrap()})
            .collect();

        for mut number in numbers {
            number.calculate_nth_iteration(2000);
            dbg!(number);

            answer += number.number;
        }
        return answer;
    }

    fn solution_two(&self, _lines: Vec<String>) -> i64 {
        return 0;
    }
}
