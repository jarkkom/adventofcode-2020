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
    xpos: i64,
    ypos: i64,
    wpx: i64,
    wpy: i64,
}

impl Ship {
    fn execute(&mut self, action: char, value: i64) {
        match action {
            'F' => {
                self.xpos += self.wpx * value;
                self.ypos += self.wpy * value;
            }
            'R' => {
                let angle = 2.0 * std::f64::consts::PI * (value as f64) / 360.0;
                let xd = (self.wpx as f64) * angle.cos() - (self.wpy as f64) * angle.sin();
                let yd = (self.wpy as f64) * angle.cos() + (self.wpx as f64) * angle.sin();
                self.wpx = xd.round() as i64;
                self.wpy = yd.round() as i64;
            }
            'L' => {
                let angle = -2.0 * std::f64::consts::PI * (value as f64) / 360.0;
                let xd = (self.wpx as f64) * angle.cos() - (self.wpy as f64) * angle.sin();
                let yd = (self.wpy as f64) * angle.cos() + (self.wpx as f64) * angle.sin();
                self.wpx = xd.round() as i64;
                self.wpy = yd.round() as i64;
            }
            'N' => {
                self.wpy -= value;
            }
            'W' => {
                self.wpx -= value;
            }
            'S' => {
                self.wpy += value;
            }
            'E' => {
                self.wpx += value;
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
        xpos: 0,
        ypos: 0,
        wpx: 10,
        wpy: -1,
    };

    for i in instructions {
        ship.execute(i.0, i.1);
    }

    println!("{} {}", ship.xpos, ship.ypos);
    println!("{} {}", ship.wpx, ship.wpy);
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
    fn test_ship_exec_rotate() {
        let mut ship = Ship {
            xpos: 0,
            ypos: 0,
            wpx: 10,
            wpy: 100,
        };

        ship.execute('L', 90);
        assert_eq!(ship.wpx, 100);
        assert_eq!(ship.wpy, -10);

        ship.execute('L', 90);
        assert_eq!(ship.wpx, -10);
        assert_eq!(ship.wpy, -100);

        ship.execute('R', 270);
        assert_eq!(ship.wpx, -100);
        assert_eq!(ship.wpy, 10);
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
            xpos: 0,
            ypos: 0,
            wpx: 10,
            wpy: -1,
        };

        ship.execute(output[0].0, output[0].1);
        assert_eq!(ship.xpos, 100);
        assert_eq!(ship.ypos, -10);

        ship.execute(output[1].0, output[1].1);
        assert_eq!(ship.xpos, 100);
        assert_eq!(ship.ypos, -10);

        ship.execute(output[2].0, output[2].1);
        assert_eq!(ship.xpos, 170);
        assert_eq!(ship.ypos, -38);

        ship.execute(output[3].0, output[3].1);
        assert_eq!(ship.xpos, 170);
        assert_eq!(ship.ypos, -38);

        ship.execute(output[4].0, output[4].1);
        assert_eq!(ship.xpos, 214);
        assert_eq!(ship.ypos, 72);

        assert_eq!(ship.manhattan(), 286);
    }
}
