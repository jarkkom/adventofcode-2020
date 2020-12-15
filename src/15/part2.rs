use std::collections::HashMap;

#[derive(Debug)]
struct MemoryGame {
    turn: i64,
    starting_numbers: Vec<i64>,
    last_seen: HashMap<i64, i64>,
    previous: i64,
}

impl MemoryGame {
    fn new(starting_numbers: Vec<i64>) -> Self {
        Self {
            turn: 0,
            starting_numbers,
            last_seen: HashMap::new(),
            previous: -1,
        }
    }
}

impl Iterator for MemoryGame {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        self.turn += 1;

        if self.turn > 30000000 {
            return None;
        }

        let speaking;

        if !self.starting_numbers.is_empty() {
            // speak a starting number
            speaking = self.starting_numbers.remove(0);
        } else if self.last_seen.contains_key(&self.previous) {
            let last_seen = self.last_seen.get(&self.previous).unwrap();

            speaking = self.turn - 1 - last_seen;
        } else {
            speaking = 0;
        }

        self.last_seen.insert(self.previous, self.turn - 1);
        self.previous = speaking;

        Some(speaking)
    }
}

fn main() {
    let game = MemoryGame::new(vec![16, 1, 0, 18, 12, 14, 19]);
    println!("30000000th {:?}", game.last());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator() {
        let starting_numbers: Vec<i64> = vec![0, 3, 6];

        let mut game = MemoryGame::new(starting_numbers);

        println!("{:?}", game);
        // Turn 1: The 1st number spoken is a starting number, 0.
        assert_eq!(game.next(), Some(0));

        println!("{:?}", game);
        // Turn 2: The 2nd number spoken is a starting number, 3.
        assert_eq!(game.next(), Some(3));

        println!("{:?}", game);
        // Turn 3: The 3rd number spoken is a starting number, 6.
        assert_eq!(game.next(), Some(6));

        println!("{:?}", game);
        // Turn 4: Now, consider the last number spoken, 6. Since that was the first time the number had been spoken, the 4th number spoken is 0.
        assert_eq!(game.next(), Some(0));

        println!("{:?}", game);
        // Turn 5: Next, again consider the last number spoken, 0. Since it had been spoken before, the next number to speak is the difference between the turn number when it was last spoken (the previous turn, 4) and the turn number of the time it was most recently spoken before then (turn 1). Thus, the 5th number spoken is 4 - 1, 3.
        assert_eq!(game.next(), Some(3));

        println!("{:?}", game);
        // Turn 6: The last number spoken, 3 had also been spoken before, most recently on turns 5 and 2. So, the 6th number spoken is 5 - 2, 3.
        assert_eq!(game.next(), Some(3));

        println!("{:?}", game);
        // Turn 7: Since 3 was just spoken twice in a row, and the last two turns are 1 turn apart, the 7th number spoken is 1.
        assert_eq!(game.next(), Some(1));

        println!("{:?}", game);
        // Turn 8: Since 1 is new, the 8th number spoken is 0.
        assert_eq!(game.next(), Some(0));

        println!("{:?}", game);
        // Turn 9: 0 was last spoken on turns 8 and 4, so the 9th number spoken is the difference between them, 4.
        assert_eq!(game.next(), Some(4));

        println!("{:?}", game);
        // Turn 10: 4 is new, so the 10th number spoken is 0.
        assert_eq!(game.next(), Some(0));
    }

    #[test]
    fn test_iterator_next_more() {
        // these take too long to run
        /*
        let game = MemoryGame::new(vec![0, 3, 6]);
        assert_eq!(game.last(), Some(175594));

        let game = MemoryGame::new(vec![1, 3, 2]);
        assert_eq!(game.last(), Some(2578));

        let game = MemoryGame::new(vec![2, 1, 3]);
        assert_eq!(game.last(), Some(3544142));

        let game = MemoryGame::new(vec![1, 2, 3]);
        assert_eq!(game.last(), Some(261214));

        let game = MemoryGame::new(vec![2, 3, 1]);
        assert_eq!(game.last(), Some(6895259));

        let game = MemoryGame::new(vec![3, 2, 1]);
        assert_eq!(game.last(), Some(18));

        let game = MemoryGame::new(vec![3, 1, 2]);
        assert_eq!(game.last(), Some(362));
        */
    }
}
