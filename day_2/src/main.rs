#![feature(iter_map_windows)]
use std::{fs, isize};

fn main() {
    let input: Vec<Vec<isize>> = parse_input("day_2/input.txt");

    let result_one = task_one(&input);
    let result_two = task_two(&input);
    println!("Task One: {}\nTask Two: {}", result_one, result_two);
}

fn task_one(input: &Vec<Vec<isize>>) -> usize {
    input.iter().filter(|x| safe_report(&x, 0)).count()
}

fn task_two(input: &Vec<Vec<isize>>) -> usize {
    input.iter().filter(|x| safe_report(&x, 1)).count()
}

fn safe_report(report: &Vec<isize>, mut fault_tolerance: isize) -> bool {
    let diff: Vec<isize> = report.iter()
        .map_windows(|[&x,&y]| (x-y)).collect();
    fault_tolerance -= diff.iter()
        .filter(|&x| x == &0 || x > &3 || x < &-3).count() as isize;
    let case_switch: Vec<isize> = diff.iter()
        .map_windows(|[&x,&y]| x * y).collect();
    fault_tolerance -= case_switch.iter()
        .filter(|&x| x < &0).count() as isize;
    fault_tolerance >= 0
}

fn parse_input(file: &str) -> Vec<Vec<isize>> {
    fs::read_to_string(file)
        .expect("could not read file")
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.split(" ")
            .map(|n| n.parse::<isize>().expect("not a number"))
            .collect()
        )
        .collect()
}
