#[derive(Debug)]
struct Device {
    state: i64,
    loops: i64,
    subject: i64,
}

impl Device {
    fn new(subject: i64) -> Self {
        Self {
            state: 1,
            loops: 0,
            subject,
        }
    }

    fn run_loop(&mut self) {
        self.state *= self.subject;
        self.state %= 20201227;
        self.loops += 1;
    }
}

fn main() {
    let mut door_1 = Device::new(7);
    let mut card_1 = Device::new(7);

    while door_1.state != 8458505 {
        door_1.run_loop();
    }

    while card_1.state != 16050997 {
        card_1.run_loop();
    }

    let mut door_2 = Device::new(card_1.state);
    let mut card_2 = Device::new(door_1.state);

    println!("door {:?}, card {:?}", door_1, card_1);

    for _ in 0..door_1.loops {
        door_2.run_loop();
    }

    for _ in 0..card_1.loops {
        card_2.run_loop();
    }

    println!("door {:?}, card {:?}", door_2, card_2);
}

mod tests {
    use super::*;

    #[test]
    fn test_get_encryption() {
        let mut door = Device::new(7);
        let mut card = Device::new(7);

        while door.state != 5764801 {
            door.run_loop();
        }
        assert_eq!(door.loops, 8);

        while card.state != 17807724 {
            card.run_loop();
        }
        assert_eq!(card.loops, 11);

        let mut door_2 = Device::new(card.state);
        for _ in 0..door.loops {
            door_2.run_loop();
        }
        assert_eq!(door_2.state, 14897079);

        let mut card_2 = Device::new(door.state);
        for _ in 0..card.loops {
            card_2.run_loop();
        }
        assert_eq!(card_2.state, door_2.state);
    }
}
