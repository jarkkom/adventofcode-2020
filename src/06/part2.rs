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

#[derive(Debug)]
struct Group {
    people: usize,
    answers: HashMap<char, usize>,
}

fn read_input(reader: impl Read) -> Result<Vec<Group>, String> {
    let reader = BufReader::new(reader);

    let mut groups: Vec<Group> = Vec::new();
    let mut curr_group_answers = HashMap::new();

    let mut curr_people = 0;

    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                if x.is_empty() {
                    groups.push(Group {
                        answers: curr_group_answers,
                        people: curr_people,
                    });
                    curr_group_answers = HashMap::new();
                    curr_people = 0;
                } else {
                    for c in x.chars() {
                        curr_group_answers.insert(c, curr_group_answers.get(&c).unwrap_or(&0) + 1);
                    }
                    curr_people += 1;
                }
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }
    if !curr_group_answers.is_empty() {
        groups.push(Group {
            answers: curr_group_answers,
            people: curr_people,
        });
    }

    Ok(groups)
}

fn count_answers(groups: Vec<Group>) -> usize {
    groups.iter().fold(0, |total, g| {
        total
            + g.answers.iter().fold(0, |answers_all_yes, (_k, v)| {
                answers_all_yes + if *v == g.people { 1 } else { 0 }
            })
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let groups = read_input(input_file.unwrap()).unwrap();

    println!("answers = {}", count_answers(groups));
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

        assert_eq!(group.people, 1);
        assert_eq!(group.answers.len(), 3);

        println!("{:?}", group);

        assert_eq!(group.answers.get(&'a').unwrap(), &1);
        assert_eq!(group.answers.get(&'b').unwrap(), &1);
        assert_eq!(group.answers.get(&'c').unwrap(), &1);

        let group3 = output.iter().nth(2).unwrap();
        println!("{:?}", group3);

        assert_eq!(group3.people, 2);
        assert_eq!(group3.answers.len(), 3);

        assert_eq!(group3.answers.get(&'a').unwrap(), &2);
        assert_eq!(group3.answers.get(&'b').unwrap(), &1);
        assert_eq!(group3.answers.get(&'c').unwrap(), &1);
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

        assert_eq!(count_answers(output), 6);
    }
}
