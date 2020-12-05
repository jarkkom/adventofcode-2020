use std::cmp;
use std::env;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

fn open_input(filename: &str) -> io::Result<File> {
    let path = Path::new(filename);
    return File::open(path);
}

fn read_input(reader: impl Read) -> Result<Vec<String>, String> {
    let reader = BufReader::new(reader);

    let mut lines: Vec<String> = Vec::new();

    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                lines.push(x);
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    return Ok(lines);
}

fn parse_row(pass: &String) -> i64 {
    return pass.chars().take(7).fold(0, |v, x| {
        return (v << 1) + if x == 'B' { 1 } else { 0 };
    });
}

fn parse_column(pass: &String) -> i64 {
    return pass.chars().skip(7).take(3).fold(0, |v, x| {
        return (v << 1) + if x == 'R' { 1 } else { 0 };
    });
}

fn get_seat_id(pass: &String) -> i64 {
    return parse_row(&pass) * 8 + parse_column(&pass);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let passes = read_input(input_file.unwrap()).unwrap();

    let max_seat_id = passes.iter().fold(0, |curr_max, pass| {
        return cmp::max(curr_max, get_seat_id(&pass));
    });
    println!("max seat id {}", max_seat_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_row() {
        assert_eq!(parse_row(&String::from("BFFFBBFRRR")), 70);
        assert_eq!(parse_row(&String::from("FFFBBBFRRR")), 14);
        assert_eq!(parse_row(&String::from("BBFFBBFRLL")), 102);
    }

    #[test]
    fn test_parse_column() {
        assert_eq!(parse_column(&String::from("BFFFBBFRRR")), 7);
        assert_eq!(parse_column(&String::from("FFFBBBFRRR")), 7);
        assert_eq!(parse_column(&String::from("BBFFBBFRLL")), 4);
    }

    #[test]
    fn test_parse_seat() {
        assert_eq!(get_seat_id(&String::from("BFFFBBFRRR")), 567);
        assert_eq!(get_seat_id(&String::from("FFFBBBFRRR")), 119);
        assert_eq!(get_seat_id(&String::from("BBFFBBFRLL")), 820);
    }
}
