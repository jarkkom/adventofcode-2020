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

fn read_input(reader: impl Read) -> Result<Vec<Vec<i64>>, String> {
    let reader = BufReader::new(reader);

    let mut groups: Vec<Vec<i64>> = Vec::new();
    let mut curr_group: Vec<i64> = Vec::new();

    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                if x.is_empty() {
                    groups.push(curr_group);
                    curr_group = Vec::new();
                } else {
                    curr_group.push(
                        x.chars()
                            .fold(0, |set, c| set | 1 << (c as u32 - 'a' as u32)),
                    );
                }
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }
    if !curr_group.is_empty() {
        groups.push(curr_group);
    }

    Ok(groups)
}

fn count_answers(groups: Vec<Vec<i64>>) -> u32 {
    groups.iter().fold(0, |c, passengers| {
        c + passengers.iter().fold(0, |union, p| union | p).count_ones()
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let groups = read_input(input_file.unwrap()).unwrap();

    println!("groups = {:?}", groups);
    println!("answers = {}", count_answers(groups));

    //println!("valid passports {:?} ", count_valid_passports(passports));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input_reader() {
        let test_input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let output = read_input(test_input.as_bytes()).unwrap();
        assert_eq!(output.len(), 5);

        let group = output.first().unwrap();

        assert_eq!(group.len(), 1);

        assert_eq!(group[0], 1 | 2 | 4);

        let group3 = output.iter().nth(2).unwrap();

        assert_eq!(group3.len(), 2);

        assert_eq!(group3[0], 1 | 2);
        assert_eq!(group3[1], 1 | 4);
    }

    #[test]
    fn test_count_valid_passports() {
        let test_input = "abc

a
b
c

ab
ac

a
a
a
a

b";
        let output = read_input(test_input.as_bytes()).unwrap();

        assert_eq!(count_answers(output), 11);
    }
}
