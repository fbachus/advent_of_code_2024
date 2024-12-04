use std::fs;

fn main() {
    let inputs = parse_input("day_1/input.txt");
    let tuplector: Vec<(usize, usize)> = inputs.iter()
        .map(|x| {
            let y: Vec<usize> = x.split("   ")
                .map(|z| z.parse::<usize>().expect("should be int"))
                .collect();
            (y[0], y[1])
        }).collect();
        
    let (mut first_col, mut second_col): (Vec<usize>, Vec<usize>) = (
        tuplector.iter().map(|x| x.0).collect(),
        tuplector.iter().map(|x| x.1).collect()
    );
    first_col.sort();
    second_col.sort();
    let result_one = task_one(&first_col, &second_col);
    let result_two = task_two(&first_col, &second_col);
    println!("result one: {}\nresult two: {}", result_one, result_two);
}

fn parse_input(file: &str) -> Vec<String> {
    let string = fs::read_to_string(file)
        .expect("could not read file");
    string.split("\n")
        .map(ToString::to_string)
        .filter(|x| x != "")
        .collect::<Vec<String>>()
}

fn task_one(first_col: &Vec<usize>, second_col: &Vec<usize>) -> usize {
    first_col
        .iter()
        .zip(second_col)
        .map(|(x,&y)| x.abs_diff(y))
        .sum()
}

fn task_two(first_col: &Vec<usize>, second_col: &Vec<usize>) -> usize {
    first_col
        .iter()
        .map(|x| x * second_col.iter().filter(|&y| y == x).count())
        .sum()
}
