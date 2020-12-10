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

fn validate_chain(input: &[i64]) -> bool {
    input
        .windows(2)
        .filter(|a| {
            println!("{} {} {}", a[0], a[1], a[1] - a[0]);
            a[1] - a[0] < 1 || a[1] - a[0] > 3
        })
        .count()
        == 0
}

fn get_deltas(input: &[i64]) -> Vec<i64> {
    input.windows(2).map(|a| a[1] - a[0]).collect()
}

fn get_answer(input: &[i64]) -> usize {
    let ones = get_deltas(input).iter().filter(|a| **a == 1).count();
    let threes = get_deltas(input).iter().filter(|a| **a == 3).count();
    println!("ones {} threes {}", ones, threes);

    // first is always 0, last is always max + 3 so increment by one
    (ones + 1) * (threes + 1)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let mut inputs = read_input(input_file.unwrap()).unwrap();
    inputs.sort_unstable();

    println!("{}", validate_chain(&inputs));

    println!("{}", get_answer(&inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> Vec<i64> {
        return vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    }

    fn get_test_data_2() -> Vec<i64> {
        return vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
    }

    #[test]
    fn test_validate_chain() {
        let mut input_1 = get_test_data();
        input_1.sort_unstable();
        assert_eq!(validate_chain(&input_1), true);

        let mut input_2 = get_test_data_2();
        input_2.sort_unstable();
        assert_eq!(validate_chain(&input_2), true);
    }

    #[test]
    fn test_get_deltas() {
        let input_1 = vec![1, 2, 3, 5, 8, 13, 21, 34, 55];
        assert_eq!(get_deltas(&input_1), vec![1, 1, 2, 3, 5, 8, 13, 21]);
    }

    #[test]
    fn test_get_jolts() {
        let mut input_1 = get_test_data();
        input_1.sort_unstable();
        assert_eq!(get_answer(&input_1), 7 * 5);

        let mut input_2 = get_test_data_2();
        input_2.sort_unstable();
        assert_eq!(get_answer(&input_2), 22 * 10);
    }
}
