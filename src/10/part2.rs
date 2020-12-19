use std::collections::HashMap;
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

fn get_arrangements(input: &[i64]) -> i64 {
    let mut cache: HashMap<i64, i64> = HashMap::new();
    cache.insert(0, 1);

    cache = input.iter().fold(cache, |mut cache, &x| {
        let p1: i64 = *cache.get(&(x - 1)).unwrap_or(&0);
        let p2: i64 = *cache.get(&(x - 2)).unwrap_or(&0);
        let p3: i64 = *cache.get(&(x - 3)).unwrap_or(&0);
        cache.insert(x, p1 + p2 + p3);
        cache
    });
    println!("{:?} cache", cache);

    let max = cache.iter().max_by_key(|(&k, _)| k);

    println!("{:?} max", max);

    *max.unwrap().1
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let mut inputs = read_input(input_file.unwrap()).unwrap();
    inputs.sort_unstable();

    println!("{}", get_arrangements(&inputs));
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
    fn test_get_arrangements() {
        let mut input_1 = get_test_data();
        input_1.sort_unstable();
        assert_eq!(get_arrangements(&input_1), 8);

        let mut input_2 = get_test_data_2();
        input_2.sort_unstable();
        assert_eq!(get_arrangements(&input_2), 19208);
    }
}
