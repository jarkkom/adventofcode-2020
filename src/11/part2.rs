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

#[derive(PartialEq, Debug)]
struct Seats {
    width: usize,
    height: usize,
    seats: Vec<i64>,
}

impl Seats {
    fn is_occupied(&self, ox: i64, oy: i64, dx: i64, dy: i64) -> i64 {
        let mut i = 1;
        loop {
            let x = ox + dx * i;
            let y = oy + dy * i;

            //println!("ox {} oy {} dx {} dy {} i {} x {} y {}", ox, oy, dx * i, dy * i, i, x, y);
            //println!("{} {}", self.width, self.height);

            i += 1;

            if x < 0 || y < 0 || x >= (self.width as i64) || y >= (self.height as i64) {
                return 0;
            }

            let idx = y as usize * self.width + x as usize;

            if self.seats[idx] == 0 {
                continue;
            }

            return if self.seats[idx] > 1 { 1 } else { 0 };
        }
    }

    fn get_visible_occupied(&self, ox: i64, oy: i64) -> i64 {
        let mut occupieds = 0;
        occupieds += self.is_occupied(ox, oy, -1, -1);
        occupieds += self.is_occupied(ox, oy, 0, -1);
        occupieds += self.is_occupied(ox, oy, 1, -1);

        occupieds += self.is_occupied(ox, oy, -1, 0);
        occupieds += self.is_occupied(ox, oy, 1, 0);

        occupieds += self.is_occupied(ox, oy, -1, 1);
        occupieds += self.is_occupied(ox, oy, 0, 1);
        occupieds += self.is_occupied(ox, oy, 1, 1);

        occupieds
    }

    fn apply_rules(&self) -> Seats {
        let mut new_seats = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let curr_state = self.seats[y * self.width + x];

                if curr_state == 0 {
                    new_seats.push(curr_state);
                    continue;
                }

                let occupieds = self.get_visible_occupied(x as i64, y as i64);

                if curr_state == 1 && occupieds == 0 {
                    new_seats.push(2);
                } else if curr_state == 2 && occupieds >= 5 {
                    new_seats.push(1);
                } else {
                    new_seats.push(curr_state);
                }
            }
        }
        Seats {
            width: self.width,
            height: self.height,
            seats: new_seats,
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.seats[y * self.width + x] {
                    2 => print!("#"),
                    1 => print!("L"),
                    0 => print!("."),
                    _ => print!("?"),
                }
            }
            println!();
        }
        println!();
    }
}

fn read_input(reader: impl Read) -> Result<Seats, String> {
    let reader = BufReader::new(reader);

    let mut seats = Vec::new();
    let mut max_width = 0;
    let mut height = 0;
    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                let mut line_width = 0;
                for c in x.chars() {
                    match c {
                        '.' => seats.push(0),
                        'L' => seats.push(1),
                        '#' => seats.push(2),
                        _ => {}
                    }
                    line_width += 1;
                }
                if line_width > max_width {
                    max_width = line_width;
                }
                height += 1;
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(Seats {
        width: max_width,
        height,
        seats,
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let init_state = read_input(input_file.unwrap()).unwrap();

    let mut curr_state = init_state;
    let mut iterations = 0;
    loop {
        let new_state = curr_state.apply_rules();
        if new_state == curr_state {
            break;
        }
        curr_state = new_state;
        iterations += 1;
    }
    curr_state.print();
    println!("iterations {}", iterations);

    println!(
        "occupied {}",
        curr_state.seats.iter().filter(|s| **s == 2).count()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> String {
        return String::from(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        );
    }

    #[test]
    fn test_read_input() {
        let map = read_input(get_test_data().as_bytes()).unwrap();

        let expected = Seats {
            width: 10,
            height: 10,
            seats: vec![
                1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1,
                0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1,
                1, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1,
                1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1,
            ],
        };

        assert_eq!(map, expected);

        map.print();
    }

    #[test]
    fn test_is_occupied_1() {
        let map1 = read_input(
            String::from(
                ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....",
            )
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(map1.get_visible_occupied(3, 4), 8);
    }

    #[test]
    fn test_is_occupied_2() {
        let map2 = read_input(
            String::from(
                ".............
.L.L.#.#.#.#.
.............",
            )
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(map2.get_visible_occupied(1, 1), 0);
    }

    #[test]
    fn test_is_occupied_3() {
        let map3 = read_input(
            String::from(
                ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.
",
            )
            .as_bytes(),
        )
        .unwrap();
        assert_eq!(map3.get_visible_occupied(3, 3), 0);
    }

    #[test]
    fn test_apply_rules() {
        let init_map = read_input(get_test_data().as_bytes()).unwrap();
        init_map.print();

        let expected_1 = read_input(
            String::from(
                "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
",
            )
            .as_bytes(),
        )
        .unwrap();

        let actual_1 = init_map.apply_rules();
        actual_1.print();
        assert_eq!(expected_1, actual_1);

        let expected_2 = read_input(
            String::from(
                "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#
",
            )
            .as_bytes(),
        )
        .unwrap();

        let actual_2 = actual_1.apply_rules();
        actual_2.print();
        assert_eq!(expected_2, actual_2);

        let expected_3 = read_input(
            String::from(
                "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#
",
            )
            .as_bytes(),
        )
        .unwrap();

        let actual_3 = actual_2.apply_rules();
        actual_3.print();
        assert_eq!(expected_3, actual_3);

        let expected_4 = read_input(
            String::from(
                "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#
",
            )
            .as_bytes(),
        )
        .unwrap();

        let actual_4 = actual_3.apply_rules();
        actual_4.print();
        assert_eq!(expected_4, actual_4);

        let expected_5 = read_input(
            String::from(
                "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
",
            )
            .as_bytes(),
        )
        .unwrap();

        let actual_5 = actual_4.apply_rules();
        actual_5.print();
        assert_eq!(expected_5, actual_5);

        let expected_6 = read_input(
            String::from(
                "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
",
            )
            .as_bytes(),
        )
        .unwrap();

        let actual_6 = actual_5.apply_rules();
        actual_6.print();
        assert_eq!(expected_6, actual_6);

        assert_eq!(actual_6.seats.iter().filter(|s| **s == 2).count(), 26);
    }
}
