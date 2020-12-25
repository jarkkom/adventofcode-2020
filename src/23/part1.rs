use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
struct Game {
    turn: i64,
    cups: VecDeque<usize>,
    current: usize,
}

impl Game {
    fn new(input: String) -> Self {
        let cups: VecDeque<usize> = input
            .chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .collect();

        let current = cups.iter().copied().next().unwrap();

        Self {
            turn: 1,
            cups,
            current,
        }
    }

    fn play_round(&mut self) {
        let mut picks = Vec::new();

        println!("-- move {:?} -- ", self.turn);

        println!("cups {:?}", self.cups);

        picks.push(self.cups.remove(1).unwrap());
        picks.push(self.cups.remove(1).unwrap());
        picks.push(self.cups.remove(1).unwrap());

        println!("pick up {:?}", picks);

        let mut destination = self.cups.get(0).unwrap() - 1;

        let dest_insert_index;
        loop {
            if let Some((ix, &iv)) = self
                .cups
                .iter()
                .enumerate()
                .find(|(_, &x)| x == destination)
            {
                dest_insert_index = ix + 1;
                destination = iv;
                break;
            } else if destination < *self.cups.iter().min().unwrap() {
                destination = *self.cups.iter().max().unwrap();
                println!("destination wraps to {:?}", destination);
            } else {
                destination -= 1;
            }
        }

        println!("will insert cups @ {:?}", dest_insert_index);

        self.cups.insert(dest_insert_index, picks[2]);
        self.cups.insert(dest_insert_index, picks[1]);
        self.cups.insert(dest_insert_index, picks[0]);

        self.cups.rotate_left(1);

        println!("destination {:?}", destination);

        self.turn += 1;
    }

    fn get_answer(&mut self) -> usize {
        let mut temp = self.cups.clone();

        while temp[0] != 1 {
            temp.rotate_left(1);
        }

        temp.iter().skip(1).fold(0, |acc, x| acc * 10 + x)
    }
}

fn main() {
    let mut game = Game::new(String::from("219347865"));

    while game.turn <= 100 {
        game.play_round();
    }

    println!("answer = {}", game.get_answer());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_new() {
        let actual = Game::new(String::from("219347865"));

        let ex_cups: VecDeque<usize> = vec![2, 1, 9, 3, 4, 7, 8, 6, 5]
            .iter()
            .map(|&x| x as usize)
            .collect();

        let expected = Game {
            cups: ex_cups,
            turn: 1,
            current: 2,
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_play_game() {
        let mut game = Game::new(String::from("389125467"));

        assert_eq!(game.cups, vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
        game.play_round();

        assert_eq!(game.cups, vec![2, 8, 9, 1, 5, 4, 6, 7, 3]);
        game.play_round();

        assert_eq!(game.cups, vec![5, 4, 6, 7, 8, 9, 1, 3, 2]);
        game.play_round();

        assert_eq!(game.cups, vec![8, 9, 1, 3, 4, 6, 7, 2, 5]);
        game.play_round();

        assert_eq!(game.cups, vec![4, 6, 7, 9, 1, 3, 2, 5, 8]);
        game.play_round();

        assert_eq!(game.cups, vec![1, 3, 6, 7, 9, 2, 5, 8, 4]);
        game.play_round();

        assert_eq!(game.cups, vec![9, 3, 6, 7, 2, 5, 8, 4, 1]);
        game.play_round();

        assert_eq!(game.cups, vec![2, 5, 8, 3, 6, 7, 4, 1, 9]);
        game.play_round();

        assert_eq!(game.cups, vec![6, 7, 4, 1, 5, 8, 3, 9, 2]);
        game.play_round();

        assert_eq!(game.cups, vec![5, 7, 4, 1, 8, 3, 9, 2, 6]);
        game.play_round();

        assert_eq!(game.cups, vec![8, 3, 7, 4, 1, 9, 2, 6, 5]);

        assert_eq!(game.get_answer(), 92658374);

        while game.turn <= 100 {
            game.play_round();
        }

        assert_eq!(game.get_answer(), 67384529);
    }
}
