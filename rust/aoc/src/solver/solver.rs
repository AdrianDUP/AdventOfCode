pub trait Solver {
    fn solution_one(&self, lines: Vec<String>) -> i64;
    fn solution_two(&self, lines: Vec<String>) -> i64;
}

pub fn find_all_by_regex(input: String, regex: &str) -> Vec<String> {
    let regx = regex::Regex::new(regex).unwrap();
    
    return regx.find_iter(&input).map(|e| String::from(e.as_str())).collect();
}

pub fn split_string_into_characters(line: String) -> Vec<String> {
    return line.split("")
        .filter(|e| *e != "")
        .map(String::from)
        .collect();
}

pub fn split_string_into_parts(line: String) -> Vec<String> {
    return line.split_whitespace()
        .filter(|x| *x != "")
        .map(String::from)
        .collect();
}

pub fn is_even_number_of_digits(number: i64) -> bool {
    return number.to_string().len() % 2 == 0;
}
