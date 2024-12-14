// Current answer with input is 2511, which is too low
use std::fs;
use std::num::TryFromIntError;

struct Position {
    x: usize,
    y: usize,
}

//#[derive(Clone)]
struct PositionChange {
    x: isize,
    y: isize,
}
struct WordAssistant<'a> {
    position: Position,
    direction: PositionChange,
    word: &'a [char],
    word_pos: usize,
}

impl Position {
    fn move_position(&mut self, change: &PositionChange) -> Result<(), TryFromIntError> {
        self.x = usize::try_from(self.x as isize + change.x)?;
        self.y = usize::try_from(self.y as isize + change.y)?;
        Ok(())
    }
}

impl WordAssistant <'_> {
    fn makes_valid_word(&mut self, field: &[Vec<char>]) -> bool {
        if self.word_pos >= self.word.len() {
            true
        } else if self.position.y >= field.len() ||
                  self.position.x >= field[self.position.y].len() {
            false
        } else if field[self.position.y][self.position.x] == self.word[self.word_pos] {
            match self.position.move_position(&self.direction) {
                Ok(_) => (),
                Err(_) => return false,
            }
            self.word_pos += 1;
            self.makes_valid_word(field)
        } else {
            false
        }
    }
}

fn main() {
    let input = parse_input("input.txt");
    //let input = parse_input("day_4/input.txt");
    let find_this = ['X', 'M', 'A', 'S'];
    let res_one = task_one(&input, &find_this);
    let res_two = task_two(&input, &find_this);
    println!("Task One: {}\nTask Two: {}", res_one, res_two);
}

fn task_one(input: &[Vec<char>], word_to_find: &[char]) -> usize {
    input.iter().enumerate().map(|(i, y)| {
        y.iter().enumerate().filter_map(|(j, x)| {
            if *x != 'X' { return None }
            // generate directions to check in 
            Some((-1..=1).map(move |move_x| {
                (-1..=1).filter(move |&move_y| {
                    if move_y == 0 && move_x == 0 { return false }
                    let mut word = WordAssistant {
                        position: Position { x: j, y: i },
                        direction: PositionChange { x: move_x, y: move_y },
                        word: word_to_find,
                        word_pos: 0,
                    };
                    word.makes_valid_word(&input)
                }).count()
            })
                //.inspect(|c| println!("{}", c))
                .sum::<usize>()
            )
        }).sum::<usize>()
    }).sum()
}

fn task_two(input: &[Vec<char>], word_to_find: &[char]) -> usize {
    //todo!();
    0
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    fs::read_to_string(input)
        .expect("Could not read file")
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

//fn search_all_directions(field: &[Vec<String>], character: String)
