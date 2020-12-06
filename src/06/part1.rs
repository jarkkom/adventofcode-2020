use std::collections::HashSet;
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

fn read_input(reader: impl Read) -> Result<Vec<HashSet<char>>, String> {
    let reader = BufReader::new(reader);

    let mut groups: Vec<HashSet<char>> = Vec::new();
    let mut curr_group = HashSet::new();

    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                if x.is_empty() {
                    groups.push(curr_group);
                    curr_group = HashSet::new();
                } else {
                    for c in x.chars() {
                        curr_group.insert(c);
                    }
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

fn count_answers(groups: Vec<HashSet<char>>) -> usize {
    groups.iter().fold(0, |c, g| c + g.len())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let groups = read_input(input_file.unwrap()).unwrap();

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

        assert_eq!(group.len(), 3);

        assert_eq!(group.contains(&'a'), true);
        assert_eq!(group.contains(&'b'), true);
        assert_eq!(group.contains(&'c'), true);

        let group3 = output.iter().nth(2).unwrap();

        assert_eq!(group3.len(), 3);

        assert_eq!(group3.contains(&'a'), true);
        assert_eq!(group3.contains(&'b'), true);
        assert_eq!(group3.contains(&'c'), true);
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
