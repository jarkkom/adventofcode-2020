use regex::Regex;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

struct Input {
    min: usize,
    max: usize,
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
                        min: m[1].parse::<usize>().unwrap(),
                        max: m[2].parse::<usize>().unwrap(),
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

    Ok(output)
}

fn validate_inputs(inputs: &[Input]) -> i64 {
    let mut valids = 0;
    for i in inputs.iter() {
        let count = i.password.chars().filter(|&c| c == i.letter).count();
        if count >= i.min && count <= i.max {
            valids += 1;
        }
    }
    valids
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    match read_input(&filename) {
        Ok(inputs) => println!("valid password: {}", validate_inputs(&inputs)),
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
                min: 1,
                max: 3,
                letter: 'a',
                password: "abcde".to_owned(),
            },
            Input {
                min: 1,
                max: 3,
                letter: 'b',
                password: "cdefg".to_owned(),
            },
            Input {
                min: 2,
                max: 9,
                letter: 'c',
                password: "ccccccccc".to_owned(),
            },
        ];
        assert_eq!(validate_inputs(&test_inputs), 2);
    }
}
