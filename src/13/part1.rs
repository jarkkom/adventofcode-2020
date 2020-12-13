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

fn read_input(reader: impl Read) -> Result<Vec<String>, String> {
    let reader = BufReader::new(reader);

    let mut lines: Vec<String> = Vec::new();

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

fn parse_lines(line: String) -> Vec<i64> {
    line.split(',')
        .filter(|&x| x != "x")
        .map(|n| n.parse().unwrap())
        .collect()
}

fn find_next_departures(arrival: i64, lines: Vec<i64>) -> Vec<(i64, i64)> {
    lines
        .iter()
        .map(|line| (*line, ((arrival / line) + 1) * line))
        .collect()
}

fn find_earliest(arrival: i64, next_deps: Vec<(i64, i64)>) -> (i64, i64) {
    *next_deps.iter().min_by_key(|(_l, d)| d - arrival).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let inputs = read_input(input_file.unwrap()).unwrap();

    let arrival: i64 = inputs[0].parse().unwrap();

    println!("arrival {}", arrival);

    let next_deps = find_next_departures(arrival, parse_lines(inputs[1].to_owned()));
    let earliest = find_earliest(arrival, next_deps);

    let wait_time = earliest.1 - arrival;

    println!(
        "earliest departure: {:?}, wait time {:?}, answer {}",
        earliest,
        wait_time,
        earliest.0 * wait_time
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let line = "7,13,x,x,59,x,31,19";
        let expected: Vec<i64> = vec![7, 13, 59, 31, 19];
        let actual = parse_lines(line.to_owned());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_earliest_departure() {
        let arrival = 939;
        let lines: Vec<i64> = vec![7, 13, 59, 31, 19];
        let expected: Vec<(i64, i64)> = vec![(7, 945), (13, 949), (59, 944), (31, 961), (19, 950)];
        let next_deps = find_next_departures(arrival, lines);
        assert_eq!(next_deps, expected);

        let earliest = find_earliest(arrival, next_deps);

        assert_eq!(earliest, (59, 944));
    }
}
