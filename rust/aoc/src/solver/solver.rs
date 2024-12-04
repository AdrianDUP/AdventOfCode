pub trait Solver {
    fn solution_one(&self, lines: Vec<String>) -> i64;
    fn solution_two(&self, lines: Vec<String>) -> i64;
}

pub fn find_all_by_regex(input: String, regex: &str) -> Vec<String> {
    let regx = regex::Regex::new(regex).unwrap();
    
    return regx.find_iter(&input).map(|e| String::from(e.as_str())).collect();
}
