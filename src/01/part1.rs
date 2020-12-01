use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::env;

fn read_input(filename: &str) -> Vec<i64> {
    let path = Path::new(filename);

    println!("{:?}", path);

    let file = File::open(path);

    println!("{:?}", file);

    let reader = BufReader::new(file.unwrap());

    let mut output = Vec::new();

    for line_iter in reader.lines() {
        let intval: i64 = line_iter.unwrap().parse().unwrap();
        output.push(intval);
    }

    return output;
}

fn find_sum(inputs: &Vec<i64>) -> i64 {
    for i in 0..inputs.len() {
        for j in i + 1..inputs.len() {
            let a = inputs.get(i).unwrap();
            let b = inputs.get(j).unwrap();
            if a + b == 2020 {
                println!("{} + {} = {} -> {}", a, b, a + b, a * b);
                return a * b;
            }
        }
    }
    return -1;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    println!("{}", filename);

    let inputs = read_input(&filename);

    find_sum(&inputs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let test_inputs: Vec<i64> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(find_sum(&test_inputs), 514579);
    }
}
