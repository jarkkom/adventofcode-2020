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

fn read_input(reader: impl Read) -> Result<Vec<(char, i64)>, String> {
    let reader = BufReader::new(reader);

    let mut instructions: Vec<(char, i64)> = Vec::new();

    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                let action = x.chars().next().unwrap();
                let val = x[1..].parse().unwrap();
                instructions.push((action, val));
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(instructions)
}

#[derive(Debug, PartialEq)]
struct Ship {
    dir: i64,
    xpos: i64,
    ypos: i64,
}

impl Ship {
    fn execute(&mut self, action: char, value: i64) {
        let dirvec: Vec<(i64, i64)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

        //println!("{} {}", action, value);

        match action {
            'F' => {
                let (xd, yd) = dirvec[self.dir as usize];
                self.xpos += xd * value;
                self.ypos += yd * value;
            }
            'R' => {
                let dirdelta = value / 90;
                self.dir = (self.dir + dirdelta) & 0x03;
            }
            'L' => {
                let dirdelta = value / 90;
                self.dir = (self.dir - dirdelta) & 0x03;
            }
            'N' => {
                self.ypos -= value;
            }
            'W' => {
                self.xpos -= value;
            }
            'S' => {
                self.ypos += value;
            }
            'E' => {
                self.xpos += value;
            }
            _ => {
                panic!("unknown action {} {}", action, value);
            }
        }
    }

    fn manhattan(&self) -> i64 {
        self.xpos.abs() + self.ypos.abs()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let instructions = read_input(input_file.unwrap()).unwrap();

    let mut ship = Ship {
        dir: 1, // east,
        xpos: 0,
        ypos: 0,
    };

    for i in instructions {
        ship.execute(i.0, i.1);
    }

    println!("{}", ship.manhattan());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_reader() {
        let test_input = "F10
N3
F7
R90
F11";
        let output = read_input(test_input.as_bytes()).unwrap();
        println!("{:?}", output);
        assert_eq!(output.len(), 5);
        assert_eq!(output[0], ('F', 10));
        assert_eq!(output[1], ('N', 3));
        assert_eq!(output[2], ('F', 7));
        assert_eq!(output[3], ('R', 90));
        assert_eq!(output[4], ('F', 11));
    }

    #[test]
    fn test_ship_exec() {
        let mut ship = Ship {
            dir: 1,
            xpos: 0,
            ypos: 0,
        };

        ship.execute('L', 90);
        assert_eq!(ship.dir, 0);
        assert_eq!(ship.xpos, 0);
        assert_eq!(ship.ypos, 0);

        ship.execute('L', 90);
        assert_eq!(ship.dir, 3);
        assert_eq!(ship.xpos, 0);
        assert_eq!(ship.ypos, 0);

        ship.execute('R', 270);
        assert_eq!(ship.dir, 2);
        assert_eq!(ship.xpos, 0);
        assert_eq!(ship.ypos, 0);

        ship.execute('N', 2);
        assert_eq!(ship.dir, 2);
        assert_eq!(ship.xpos, 0);
        assert_eq!(ship.ypos, -2);

        ship.execute('S', 2);
        assert_eq!(ship.dir, 2);
        assert_eq!(ship.xpos, 0);
        assert_eq!(ship.ypos, 0);

        ship.execute('E', 2);
        assert_eq!(ship.dir, 2);
        assert_eq!(ship.xpos, 2);
        assert_eq!(ship.ypos, 0);

        ship.execute('W', 2);
        assert_eq!(ship.dir, 2);
        assert_eq!(ship.xpos, 0);
        assert_eq!(ship.ypos, 0);
    }

    #[test]
    fn test_ship_exec_2() {
        let test_input = "F10
N3
F7
R90
F11";
        let output = read_input(test_input.as_bytes()).unwrap();

        let mut ship = Ship {
            dir: 1,
            xpos: 0,
            ypos: 0,
        };

        ship.execute(output[0].0, output[0].1);
        assert_eq!(ship.xpos, 10);
        assert_eq!(ship.ypos, 0);

        ship.execute(output[1].0, output[1].1);
        assert_eq!(ship.xpos, 10);
        assert_eq!(ship.ypos, -3);

        ship.execute(output[2].0, output[2].1);
        assert_eq!(ship.xpos, 17);
        assert_eq!(ship.ypos, -3);

        ship.execute(output[3].0, output[3].1);
        assert_eq!(ship.dir, 2);
        assert_eq!(ship.xpos, 17);
        assert_eq!(ship.ypos, -3);

        ship.execute(output[4].0, output[4].1);
        assert_eq!(ship.xpos, 17);
        assert_eq!(ship.ypos, 8);

        assert_eq!(ship.manhattan(), 25);
    }
}
