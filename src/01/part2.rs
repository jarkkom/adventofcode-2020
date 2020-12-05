use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn read_input(filename: &str) -> Result<Vec<i64>, String> {
    let path = Path::new(filename);
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());

    let mut output = Vec::new();
    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                match x.parse::<i64>() {
                    Ok(num) => output.push(num),
                    Err(err) => return Err(format!("invalid number {:?}, {:?}", x, err)),
                }
                output.push(x.parse().unwrap());
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    return Ok(output);
}

fn find_sum(inputs: &Vec<i64>) -> i64 {
    for (i, a) in inputs.iter().enumerate() {
        for (j, b) in inputs.iter().skip(i).enumerate() {
            for c in inputs.iter().skip(i + j) {
                if a + b + c == 2020 {
                    println!("{} + {} + {} = {} -> {}", a, b, c, a + b + c, a * b * c);
                    return a * b * c;
                }
            }
        }
    }
    return -1;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    match read_input(&filename) {
        Ok(inputs) => println!("sum is {}", find_sum(&inputs)),
        Err(err) => println!("could not parse input {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let test_inputs: Vec<i64> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(find_sum(&test_inputs), 241861950);
    }
}
