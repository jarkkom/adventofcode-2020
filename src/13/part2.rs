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

fn parse_lines(line: String) -> Vec<(i64, i64)> {
    line.split(',')
        .enumerate()
        .filter(|(_i, x)| *x != "x")
        .map(|(i, n)| (i as i64, n.parse().unwrap()))
        .collect()
}

fn find_earliest(lines: &[(i64, i64)]) -> i64 {
    let mut t = 0;

    // start stepping by using first line
    let mut step = lines[0].1;

    // for following each increment & line
    for (i, x) in lines.iter().skip(1) {
        println!(
            "t = {}, starting search increment {} line {} step {}",
            t, i, x, step
        );

        // while current time + increment is not divisible by x
        while (t + i) % x != 0 {
            // step to next possible time
            t += step;
        }

        println!(
            "t = {}, was multiple for x {} i {}, next step {}",
            t,
            i,
            x,
            step * x
        );

        // next time has to be at least multiple of previous ones
        step *= x;
    }
    t
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let inputs = read_input(input_file.unwrap()).unwrap();

    let lines = parse_lines(inputs[1].to_owned());

    println!("lines {:?}", lines);

    let t = find_earliest(&lines);

    println!("t = {}", t);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let line = "7,13,x,x,59,x,31,19";
        let expected: Vec<(i64, i64)> = vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)];
        let actual = parse_lines(line.to_owned());

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_earliest() {
        assert_eq!(
            find_earliest(&parse_lines(String::from("7,13,x,x,59,x,31,19"))),
            1068781
        );
        assert_eq!(
            find_earliest(&parse_lines(String::from("17,x,13,19"))),
            3417
        );
        assert_eq!(
            find_earliest(&parse_lines(String::from("67,7,59,61"))),
            754018
        );
        assert_eq!(
            find_earliest(&parse_lines(String::from("67,x,7,59,61"))),
            779210
        );
        assert_eq!(
            find_earliest(&parse_lines(String::from("67,7,x,59,61"))),
            1261476
        );
        assert_eq!(
            find_earliest(&parse_lines(String::from("1789,37,47,1889"))),
            1202161486
        );
    }
}
