use regex::Regex;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

struct Input {
    pos1: usize,
    pos2: usize,
    letter: char,
    password: String,
}

fn read_input(filename: &str) -> Result<Vec<Input>, String> {
    let path = Path::new(filename);
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());

    let re = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();

    let mut output = Vec::new();
    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                let capture_iter = re.captures_iter(x.as_str());
                for m in capture_iter {
                    let i = Input {
                        pos1: m[1].parse::<usize>().unwrap(),
                        pos2: m[2].parse::<usize>().unwrap(),
                        letter: m[3].chars().next().unwrap(),
                        password: m[4].to_owned(),
                    };

                    output.push(i);
                }
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    return Ok(output);
}

fn validate_inputs(inputs: Vec<Input>) -> i64 {
    return inputs.iter().fold(0, |valids: i64, i: &Input| -> i64 {
        let c1 = i.password.chars().skip(i.pos1 - 1).next().unwrap();
        let c2 = i.password.chars().skip(i.pos2 - 1).next().unwrap();
        if (c1 == i.letter) ^ (c2 == i.letter) {
            return valids + 1;
        } else {
            return valids;
        };
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    match read_input(&filename) {
        Ok(inputs) => println!("valid password: {}", validate_inputs(inputs)),
        Err(err) => println!("could not parse input {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_inputs() {
        let test_inputs: Vec<Input> = vec![
            Input {
                pos1: 1,
                pos2: 3,
                letter: 'a',
                password: "abcde".to_owned(),
            },
            Input {
                pos1: 1,
                pos2: 3,
                letter: 'b',
                password: "cdefg".to_owned(),
            },
            Input {
                pos1: 2,
                pos2: 9,
                letter: 'c',
                password: "ccccccccc".to_owned(),
            },
        ];
        assert_eq!(validate_inputs(test_inputs), 1);
    }
}
