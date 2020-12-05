use std::io;
use std::io::Read;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::env;

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
    let mut v = 0;
    for x in pass.chars().take(7) {
        v <<= 1;
        v += if x == 'B' { 1 } else { 0 };
    }
    return v;
}

fn parse_column(pass: &String) -> i64 {
    let mut v = 0;
    for x in pass.chars().skip(7).take(3) {
        v <<= 1;
        v += if x == 'R' { 1 } else { 0 };
    }
    return v;
}

fn get_seat_id(pass: &String) -> i64 {
    return parse_row(&pass) * 8 + parse_column(&pass);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let passes = read_input(input_file.unwrap()).unwrap();

    let mut seat_ids: Vec<i64> = passes.iter()
        .map(|pass| return get_seat_id(&pass)).collect();

    seat_ids.sort();

    println!("{:?}", seat_ids.windows(2)
        .filter(|ids| return ids[1] - ids[0] > 1)
        .next());
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
