use std::collections::vec_deque::VecDeque;
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

fn read_input(reader: impl Read) -> Result<Vec<i64>, String> {
    let reader = BufReader::new(reader);

    let mut lines: Vec<i64> = Vec::new();

    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                lines.push(x.parse().unwrap());
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(lines)
}

fn find_first_not_sum(previous: &mut VecDeque<i64>, input: VecDeque<i64>) -> Option<i64> {
    for i in input {
        if !is_sum(&previous, i) {
            return Some(i);
        }
        previous.pop_front();
        previous.push_back(i);
    }
    None
}

fn is_sum(previous: &VecDeque<i64>, n: i64) -> bool {
    for (i, p1) in previous.iter().enumerate() {
        for p2 in previous.iter().skip(i + 1) {
            if p1 + p2 == n {
                return true;
            }
        }
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let mut inputs = read_input(input_file.unwrap())
        .unwrap()
        .into_iter()
        .collect::<VecDeque<i64>>();

    let preamble = 25;

    let mut previous: VecDeque<i64> = inputs.drain(..preamble).collect();

    println!("{:?}", find_first_not_sum(&mut previous, inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> Vec<i64> {
        return vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
    }

    #[test]
    fn test_find_sum1() {
        let mut v: VecDeque<_> = VecDeque::new();

        for i in 1..25 + 1 {
            v.push_back(i);
        }

        assert_eq!(is_sum(&v, 26), true);
        assert_eq!(is_sum(&v, 49), true);
        assert_eq!(is_sum(&v, 100), false);
        assert_eq!(is_sum(&v, 50), false);
    }

    #[test]
    fn test_find_sum2() {
        let mut test_data = get_test_data().into_iter().collect::<VecDeque<i64>>();

        let preamble = 5;

        let mut previous: VecDeque<i64> = test_data.drain(..preamble).collect();

        println!("{:?}", previous);

        assert_eq!(find_first_not_sum(&mut previous, test_data), Some(127));
    }
}
