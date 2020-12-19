use regex::Regex;
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

#[derive(Debug, PartialEq)]
enum Rule {
    Char(String),
    Concat(Vec<usize>),
    Union(Vec<usize>, Vec<usize>),
}

fn parse_rules(s: String) -> (usize, Rule) {
    let parts: Vec<&str> = s.split(':').collect();

    let index: usize = parts[0].parse().unwrap();

    let rule_str = parts[1];

    let char_re = Regex::new(r#""(.+)""#).unwrap();

    if let Some(captures) = char_re.captures(rule_str) {
        println!("char {:?}", captures);
        return (index, Rule::Char(captures[1].to_owned()));
    }

    if rule_str.contains('|') {
        let or_parts: Vec<&str> = rule_str.split('|').collect();

        return (
            index,
            Rule::Union(
                or_parts[0]
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
                or_parts[1]
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            ),
        );
    } else {
        return (
            index,
            Rule::Concat(
                rule_str
                    .split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            ),
        );
    }
}

fn read_input(reader: impl Read) -> Result<(HashMap<usize, Rule>, Vec<String>), String> {
    let reader = BufReader::new(reader);

    let mut rules: HashMap<usize, Rule> = HashMap::new();

    let mut line_iter = reader.lines();

    while let Some(s) = line_iter.next() {
        match s {
            Ok(s) => {
                if s.is_empty() {
                    break;
                }
                let (ix, rule) = parse_rules(s);
                rules.insert(ix, rule);
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    let mut messages = Vec::new();

    for s in line_iter {
        match s {
            Ok(s) => {
                messages.push(s);
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok((rules, messages))
}

fn traverse_rules(rules: &HashMap<usize, Rule>, pos: usize) -> String {
    let mut regex = String::from("");

    match &rules[&pos] {
        Rule::Char(s) => {
            regex.push_str(s);
        }
        Rule::Concat(a) => {
            a.iter()
                .for_each(|r| regex.push_str(&traverse_rules(rules, *r)));
        }
        Rule::Union(a, b) => {
            regex.push('(');
            a.iter()
                .for_each(|r| regex.push_str(&traverse_rules(rules, *r)));
            regex.push('|');
            b.iter()
                .for_each(|r| regex.push_str(&traverse_rules(rules, *r)));
            regex.push(')');
        }
    }

    regex
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let inputs = read_input(input_file.unwrap()).unwrap();

    println!("rules = {:?}", inputs);

    let regex_str = format!("^{}$", traverse_rules(&inputs.0, 0));
    println!("regex = {}", regex_str);

    let regex = Regex::new(&regex_str).unwrap();

    let matching = inputs.1.iter().filter(|i| regex.is_match(i)).count();
    println!("answer = {}", matching);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_reader() {
        let test_input = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;
        let output = read_input(test_input.as_bytes()).unwrap();

        println!("output = {:?}", output);

        let regex_str = format!("^{}$", traverse_rules(&output.0, 0));
        println!("regex = {}", regex_str);

        let regex = Regex::new(&regex_str).unwrap();

        let matching = output.1.iter().filter(|i| regex.is_match(i)).count();
        assert_eq!(matching, 2);
    }
}
