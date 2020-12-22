use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

fn open_input(filename: &str) -> io::Result<File> {
    let path = Path::new(filename);
    File::open(path)
}

fn read_input(reader: impl Read) -> Result<(VecDeque<i64>, VecDeque<i64>), String> {
    let reader = BufReader::new(reader);

    let mut stack_1: VecDeque<i64> = VecDeque::new();
    let mut stack_2: VecDeque<i64> = VecDeque::new();

    let mut lines_iter = reader.lines();

    while let Some(line) = lines_iter.next() {
        match line {
            Ok(x) => {
                if x.is_empty() {
                    break;
                }

                if let Ok(n) = x.parse() {
                    stack_1.push_back(n);
                }
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    while let Some(line) = lines_iter.next() {
        match line {
            Ok(x) => {
                if x.is_empty() {
                    break;
                }

                if let Ok(n) = x.parse() {
                    stack_2.push_back(n);
                }
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok((stack_1, stack_2))
}

struct Game {
    stack_1: VecDeque<i64>,
    stack_2: VecDeque<i64>,
}

impl Game {
    fn new(stack_1: VecDeque<i64>, stack_2: VecDeque<i64>) -> Game {
        Game { stack_1, stack_2 }
    }

    fn play_round(&mut self) {
        let n1 = self.stack_1.pop_front().unwrap();
        let n2 = self.stack_2.pop_front().unwrap();

        if n1 > n2 {
            self.stack_1.push_back(n1);
            self.stack_1.push_back(n2);
        } else {
            self.stack_2.push_back(n2);
            self.stack_2.push_back(n1);
        }
    }

    fn is_finished(&self) -> bool {
        self.stack_1.is_empty() || self.stack_2.is_empty()
    }

    fn winning_score(&self) -> i64 {
        let full_stack: &VecDeque<i64>;

        if self.stack_1.is_empty() {
            full_stack = &self.stack_2;
        } else {
            full_stack = &self.stack_1;
        }

        full_stack
            .iter()
            .rev()
            .enumerate()
            .fold(0, |total, (i, card)| total + (i + 1) as i64 * card)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename).unwrap();

    let (stack_1, stack_2) = read_input(input_file).unwrap();

    println!("stack_1 {:?}", stack_1);
    println!("stack_2 {:?}", stack_2);

    let mut game = Game::new(stack_1, stack_2);
    while !game.is_finished() {
        game.play_round();
    }

    let score = game.winning_score();
    println!("{:?}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_rules() {
        let test_input = get_test_input();

        let (stack_1, stack_2) = read_input(test_input.as_bytes()).unwrap();

        assert_eq!(stack_1, vec![9, 2, 6, 3, 1]);
        assert_eq!(stack_2, vec![5, 8, 4, 7, 10]);
    }

    #[test]
    fn test_play_game() {
        let test_input = get_test_input();

        let (stack_1, stack_2) = read_input(test_input.as_bytes()).unwrap();

        let mut game = Game::new(stack_1, stack_2);

        game.play_round();
        assert_eq!(game.stack_1, vec![2, 6, 3, 1, 9, 5]);
        assert_eq!(game.stack_2, vec![8, 4, 7, 10]);

        game.play_round();
        assert_eq!(game.stack_1, vec![6, 3, 1, 9, 5]);
        assert_eq!(game.stack_2, vec![4, 7, 10, 8, 2]);

        game.play_round();
        assert_eq!(game.stack_1, vec![3, 1, 9, 5, 6, 4]);
        assert_eq!(game.stack_2, vec![7, 10, 8, 2]);

        while !game.is_finished() {
            game.play_round();
        }

        let score = game.winning_score();
        assert_eq!(score, 306);
    }

    fn get_test_input() -> String {
        String::from(
            "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10",
        )
    }
}
