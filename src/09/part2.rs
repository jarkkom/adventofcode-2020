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

// find position and length
fn find_contiguous_sum(previous: &VecDeque<i64>, n: i64) -> Option<(usize, usize)> {
    for (i, _p1) in previous.iter().enumerate() {
        let mut sum = 0;
        for (i2, p2) in previous.iter().skip(i).enumerate() {
            sum += p2;
            //println!("[{}, {}] sum {}", i, i + i2 + 1, sum);
            if sum == n {
                return Some((i, i2 + 1));
            }
            if sum > n {
                break;
            }
        }
    }
    None
}

fn find_sum_range(
    previous: &mut VecDeque<i64>,
    input: VecDeque<i64>,
    n: i64,
) -> Option<(usize, usize)> {
    for i in input {
        if let Some(m) = find_contiguous_sum(&previous, n) {
            return Some(m);
        }
        previous.pop_front();
        previous.push_back(i);
    }
    None
}

fn find_min_max_sum(previous: &mut VecDeque<i64>, input: VecDeque<i64>, n: i64) -> i64 {
    let (start, len) = find_sum_range(previous, input, n).unwrap();

    println!("{:?}, {} {}", previous, start, len);

    let min = previous.iter().skip(start).take(len).min().unwrap();
    let max = previous.iter().skip(start).take(len).max().unwrap();

    println!("{} {} {}", min, max, min + max);

    min + max
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

    println!("{}", find_min_max_sum(&mut previous, inputs, 507622668));
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
    fn test_find_contiguous_sum() {
        let test_data = get_test_data().into_iter().collect::<VecDeque<i64>>();

        assert_eq!(find_contiguous_sum(&test_data, 127), Some((2, 4)));
    }

    #[test]
    fn test_find_sum_range() {
        let mut test_data = get_test_data().into_iter().collect::<VecDeque<i64>>();

        let preamble = 5;

        let mut previous: VecDeque<i64> = test_data.drain(..preamble).collect();

        let (start, len) = find_sum_range(&mut previous, test_data, 127).unwrap();

        println!("{} {}", start, len);

        let range: Vec<i64> = previous.iter().skip(start).take(len).cloned().collect();

        assert_eq!(range, vec![15, 25, 47, 40]);
    }

    #[test]
    fn test_find_min_max_sum() {
        let mut test_data = get_test_data().into_iter().collect::<VecDeque<i64>>();

        let preamble = 5;

        let mut previous: VecDeque<i64> = test_data.drain(..preamble).collect();

        println!("{:?}", previous);

        assert_eq!(find_min_max_sum(&mut previous, test_data, 127), 62);
    }
}
