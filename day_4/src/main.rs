use std::fs;
use std::num::TryFromIntError;

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct PositionChange {
    x: isize,
    y: isize,
}
#[derive(Debug)]
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
        if self.position.y >= field.len() ||
                  self.position.x >= field[self.position.y].len() {
            false
        } else if field[self.position.y][self.position.x] == self.word[self.word_pos] {
            self.word_pos += 1;
            if self.word_pos >= self.word.len() {
                return true
            }
            else if self.position.move_position(&self.direction).is_err() { 
                return false
            }
            self.makes_valid_word(field)
        } else {
            false
        }
    }
}

fn main() {
    let input = parse_input("day_4/input.txt");
    //let find_this = ['X', 'M', 'A', 'S'];
    let find_this: Vec<char> = "XMAS".chars().collect();
    let res_one = task_one(&input, &find_this);
    let res_two = task_two(&input, &find_this[1..4]);
    println!("Task One: {}\nTask Two: {}", res_one, res_two);
}

fn task_one(input: &[Vec<char>], word_to_find: &[char]) -> usize {
    input.iter().enumerate().map(|(i, y)| {
        y.iter().enumerate()
            .filter(|(_, x)| x == &&word_to_find[0])
            .map(|(j,_)| { // generate directions to check in 
                (-1..=1).map(move |move_y| {
                    (-1..=1).map(move |move_x| (move_x, move_y))
                })
                .flatten()
                .filter(move |(move_x, move_y)| {
                    if move_y == &0 && move_x == &0 { return false }
                    let mut word = WordAssistant {
                        position: Position { x: j, y: i },
                        direction: PositionChange { x: *move_x, y: *move_y },
                        word: word_to_find,
                        word_pos: 0,
                    };
                    word.makes_valid_word(input)
                }).count()  // count words found on one position
            }).sum::<usize>() // sum up characters of one line
    }).sum::<usize>() // sum up lines
}

fn task_two(input: &[Vec<char>], word_to_find: &[char]) -> usize {
    let anchor = 1;
    println!("{}", input.len());
    input.iter().enumerate()
        .filter(|(i, _)|
            *i > 0 &&
            *i < input.len() - 1)
        .map(|(i,y)| {
            y.iter().enumerate()
                .filter(|(j, x)|
                    **x == word_to_find[anchor] &&
                    *j > 0 &&
                    *j < input[i].len() - 1)
                .filter(|(j,_)| {
                    [(-1, 1), (1, 1)].iter()
                        .filter(move |(move_x, move_y)| {
                            check_surrounding(input, (*j, i), (*move_x, *move_y))
                        }).count() > 1
                }).count()
        }).sum::<usize>()
}

fn check_surrounding(field: &[Vec<char>], pos: (usize, usize), dir: (isize, isize)) -> bool{
    let other_chars = ['M', 'S'];
    let new_pos = |p: usize, d: isize, m: isize| {
        usize::try_from(p as isize + d * m).unwrap()
    };
    [[0,1], [1,0]].iter() // check 'M' and 'S'
        // zip, to multiply one of the conditions with -1, to check that
        // opposite the 'M' will be an 'S'
        .any(|i| std::iter::zip(i, [1,-1])
            .all(|(j,m)| other_chars[*j] == field
                [new_pos(pos.1, dir.1, m)] 
                [new_pos(pos.0, dir.0, m)]
            )
        )
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    fs::read_to_string(input)
        .expect("Could not read file")
        .trim()
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

//fn search_all_directions(field: &[Vec<String>], character: String)
