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
struct Rule {
    bag: String,
    count: i64,
}

fn parse_rule(rule: &str) -> Option<(&str, Vec<Rule>)> {
    let rule_re = Regex::new(r"^(.*) bags contain (.*).$").unwrap();
    if let Some(rule_capt) = rule_re.captures(rule) {
        if rule_capt.len() != 3 {
            return None;
        };

        let target = rule_capt.get(1).map_or("", |m| m.as_str());
        let contains = rule_capt.get(2).map_or("", |m| m.as_str());

        let inner = contains
            .split(',')
            .map(|i| {
                let inner_re = Regex::new(r"(\d+) (.*) bag").unwrap();

                if let Some(inner_capt) = inner_re.captures(i) {
                    return Rule {
                        bag: inner_capt.get(2).map_or("", |m| m.as_str()).to_owned(),
                        count: inner_capt
                            .get(1)
                            .map_or("", |m| m.as_str())
                            .parse()
                            .unwrap_or(0),
                    };
                }
                Rule {
                    bag: String::from(""),
                    count: 0,
                }
            })
            .collect();

        return Some((target, inner));
    }
    None
}

fn read_input(reader: impl Read) -> Result<HashMap<String, Vec<Rule>>, String> {
    let reader = BufReader::new(reader);

    let mut rules: HashMap<String, Vec<Rule>> = HashMap::new();

    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                let (outer, inner) = parse_rule(&x).unwrap();
                rules.insert(outer.to_owned(), inner);
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(rules)
}

fn traverse_rules(rules: &HashMap<String, Vec<Rule>>, color: &str, count: i64) -> i64 {
    let mut c = count;
    if let Some(s) = rules.get(color) {
        for contained in s {
            c += contained.count * traverse_rules(rules, &contained.bag, 1);
        }
    }
    println!("returning from {:?} {}", color, c);

    c
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let rules = read_input(input_file.unwrap()).unwrap();

    let count = traverse_rules(&rules, &String::from("shiny gold"), 0);
    println!("{:?}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_reader() {
        let test_input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let output = read_input(test_input.as_bytes()).unwrap();
        //println!("{:?}", output);
        assert_eq!(
            output["light red"],
            vec![
                Rule {
                    bag: String::from("bright white"),
                    count: 1
                },
                Rule {
                    bag: String::from("muted yellow"),
                    count: 2
                }
            ]
        );
    }

    #[test]
    fn test_parse_rule() {
        let (outer, inners) =
            parse_rule("light red bags contain 1 bright white bag, 2 muted yellow bags.").unwrap();
        assert_eq!(outer, "light red");
        assert_eq!(
            inners,
            vec![
                Rule {
                    bag: String::from("bright white"),
                    count: 1
                },
                Rule {
                    bag: String::from("muted yellow"),
                    count: 2
                }
            ]
        );
    }

    #[test]
    fn test_traverse_rules() {
        let test_input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let output = read_input(test_input.as_bytes()).unwrap();
        let count = traverse_rules(&output, &String::from("shiny gold"), 0);
        assert_eq!(count, 32);
    }

    #[test]
    fn test_traverse_rules_2() {
        let test_input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        let output = read_input(test_input.as_bytes()).unwrap();
        let count = traverse_rules(&output, &String::from("shiny gold"), 0);
        assert_eq!(count, 126);
    }
}
