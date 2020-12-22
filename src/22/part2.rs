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

fn read_input(reader: impl Read) -> Result<(VecDeque<usize>, VecDeque<usize>), String> {
    let reader = BufReader::new(reader);

    let mut stack_1: VecDeque<usize> = VecDeque::new();
    let mut stack_2: VecDeque<usize> = VecDeque::new();

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

#[derive(Debug)]
struct Game {
    stack_1: VecDeque<usize>,
    stack_2: VecDeque<usize>,
    cache_1: Vec<VecDeque<usize>>,
    cache_2: Vec<VecDeque<usize>>,
    round: usize,
    debug: bool,
}

impl Game {
    fn new(stack_1: VecDeque<usize>, stack_2: VecDeque<usize>) -> Game {
        Game {
            stack_1,
            stack_2,
            cache_1: Vec::new(),
            cache_2: Vec::new(),
            round: 1,
            debug: false,
        }
    }

    // return true if 1 was winner, false if 2
    fn play(&mut self) -> bool {
        loop {
            if self.is_cached() {
                return true;
            }

            self.cache_1.push(self.stack_1.clone());
            self.cache_2.push(self.stack_2.clone());

            let winner = self.play_round();

            if self.is_finished() {
                return winner;
            }
        }
    }

    // return true if 1 was winner, false if 2
    fn play_round(&mut self) -> bool {
        if self.debug {
            println!("-- Round {} (Game x) --", self.round);
            println!("Player 1's deck: {:?}", self.stack_1);
            println!("Player 2's deck: {:?}", self.stack_2);
        }

        let n1 = self.stack_1.pop_front().unwrap();
        let n2 = self.stack_2.pop_front().unwrap();

        if self.debug {
            println!("Player 1 plays: {:?}", n1);
            println!("Player 2 plays: {:?}", n2);
        }

        let is_one_winner;
        if self.stack_1.len() >= n1 && self.stack_2.len() >= n2 {
            let mut new_game = Game::new(
                self.stack_1.iter().copied().take(n1).collect(),
                self.stack_2.iter().copied().take(n2).collect(),
            );
            if self.debug {
                println!("Playing a sub-game to determine the winner...");
            }

            is_one_winner = new_game.play();

            if self.debug {
                println!(
                    "The winner of subgame is player {:?}",
                    if is_one_winner { 1 } else { 2 }
                );
            }
        } else {
            is_one_winner = n1 > n2;
        }

        if self.debug {
            println!(
                "Player {:?} wins round {}",
                if is_one_winner { 1 } else { 2 },
                self.round
            );
            println!();
        }

        self.round += 1;

        if is_one_winner {
            self.stack_1.push_back(n1);
            self.stack_1.push_back(n2);
        } else {
            self.stack_2.push_back(n2);
            self.stack_2.push_back(n1);
        }

        is_one_winner
    }

    fn is_cached(&self) -> bool {
        self.cache_1.contains(&self.stack_1) || self.cache_2.contains(&self.stack_2)
    }

    fn is_finished(&self) -> bool {
        self.stack_1.is_empty() || self.stack_2.is_empty()
    }

    fn winning_score(&self) -> usize {
        let full_stack: &VecDeque<usize>;

        if self.stack_1.is_empty() {
            full_stack = &self.stack_2;
        } else {
            full_stack = &self.stack_1;
        }

        full_stack
            .iter()
            .rev()
            .enumerate()
            .fold(0, |total, (i, card)| total + (i + 1) * card)
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

    game.play();

    println!("end state {:?} {:?}", game.stack_1, game.stack_2);

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

        assert_eq!(game.play_round(), true);
        assert_eq!(game.stack_1, vec![2, 6, 3, 1, 9, 5]);
        assert_eq!(game.stack_2, vec![8, 4, 7, 10]);

        assert_eq!(game.play_round(), false);
        assert_eq!(game.stack_1, vec![6, 3, 1, 9, 5]);
        assert_eq!(game.stack_2, vec![4, 7, 10, 8, 2]);

        assert_eq!(game.play_round(), true);
        assert_eq!(game.stack_1, vec![3, 1, 9, 5, 6, 4]);
        assert_eq!(game.stack_2, vec![7, 10, 8, 2]);

        while !game.is_finished() {
            game.play_round();
        }

        println!("end state {:?}", game);

        let score = game.winning_score();
        assert_eq!(score, 291);
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
