use std::fs;
use regex::Regex;

fn main() {
    let input = parse_input("day_3/input.txt");
    let instruction_pattern = Regex::new(r"(do(?:n't)*)*.*mul\((\d{1,4}),(\d{1,4})\)").unwrap();
    let mut instructions = vec![];
    let enable = true;
    for (_, [ena, num1, num2]) in instruction_pattern.captures_iter(&input[..]).map(|c| c.extract()) {
        println!("{}", ena );
        // match toggle {
        //     Some(x) => enable = x,
        //     None => (),
        // }
        instructions.push((
            ena,
            num1.parse::<usize>().expect("NaN"),
            num2.parse::<usize>().expect("NaN")
        ));
    }
    println!("{:?}", instructions);
    // for x in &instructions {
    //     println!("{:?}", x);
    // }
    let result_one: usize = instructions.iter().map(|(d,x,y)| x * y).sum();
    let result_two = 0;
    println!("Task One: {}\nTask Two: {}", result_one, result_two);
}

fn parse_input(file: &str) -> String {
    fs::read_to_string(file).expect("Could not read file")
}
