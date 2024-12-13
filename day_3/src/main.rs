use std::fs;
use regex::Regex;

fn main() {
    let input = parse_input("day_3/input.txt");
    let instruction_pattern = Regex::new(r"mul\((\d{1,4}),(\d{1,4})\)").unwrap();

    let filtered_string = get_valid_strings(&input);
    let result_one: usize = evaluate_instructions(&input, &instruction_pattern);
    let result_two = evaluate_instructions(&filtered_string, &instruction_pattern);
    println!("Task One: {}\nTask Two: {}", result_one, result_two);
}

fn parse_input(file: &str) -> String {
    fs::read_to_string(file).expect("Could not read file")
}

fn get_valid_strings(input: &str) -> String {
    let starts_as_do = input.split_once("don't").unwrap();

    let filtered = starts_as_do.1
        .split("don't")
        .map(|x| { 
            x.split("do")
                .map(String::from)
                .skip(1)
                .collect::<String>()
        })
        .collect();
    [String::from(starts_as_do.0), filtered].concat()
}

fn evaluate_instructions(input: &str, match_pattern: &Regex) -> usize {
    let mut instructions = vec![];
    for (_, [num1, num2]) in match_pattern.captures_iter(input)
        .map(|c| c.extract()) {
        instructions.push((
            num1.parse::<usize>().expect("NaN"),
            num2.parse::<usize>().expect("NaN")
        ));
    }
    instructions.iter().map(|(x,y)| x * y).sum()
}
