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

fn traverse_rules(rules: &HashMap<usize, Rule>, pos: usize, depth: usize) -> String {
    let mut regex = String::from("");

    if depth > 20 {
        return regex;
    }

    match &rules[&pos] {
        Rule::Char(s) => {
            regex.push_str(s);
        }
        Rule::Concat(a) => {
            a.iter()
                .for_each(|r| regex.push_str(&traverse_rules(rules, *r, depth + 1)));
        }
        Rule::Union(a, b) => {
            regex.push('(');
            a.iter()
                .for_each(|r| regex.push_str(&traverse_rules(rules, *r, depth + 1)));
            regex.push('|');
            b.iter()
                .for_each(|r| regex.push_str(&traverse_rules(rules, *r, depth + 1)));
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

    let regex_str = format!("^{}$", traverse_rules(&inputs.0, 0, 0));
    println!("regex = {}", regex_str);

    let regex = Regex::new(&regex_str).unwrap();

    let matching = inputs.1.iter().filter(|i| regex.is_match(i)).count();
    println!("answer = {}", matching);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_recursive() {
        let test_input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
        let output = read_input(test_input.as_bytes()).unwrap();

        println!("output = {:?}", output);

        let regex_str = format!("^{}$", traverse_rules(&output.0, 0, 0));
        println!("regex = {}", regex_str);

        let regex = Regex::new(&regex_str).unwrap();

        let matching = output.1.iter().filter(|i| regex.is_match(i)).count();
        assert_eq!(matching, 3);
    }

    #[test]
    fn test_recursive() {
        let test_input = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31 | 42 11 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42 | 42 8
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
        let output = read_input(test_input.as_bytes()).unwrap();

        println!("output = {:?}", output);

        let regex_str = format!("^{}$", traverse_rules(&output.0, 0, 0));
        println!("regex = {}", regex_str);

        let regex = Regex::new(&regex_str).unwrap();

        let matching = output.1.iter().filter(|i| regex.is_match(i)).count();
        assert_eq!(matching, 12);
    }

}
