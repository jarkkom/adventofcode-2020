use regex::Regex;
use std::collections::HashMap;
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

fn parse_rule(rule: &str) -> Option<(&str, Vec<&str>)> {
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
                    return inner_capt.get(2).map_or("", |m| m.as_str());
                }
                ""
            })
            .collect();

        return Some((target, inner));
    }
    None
}

fn read_input(reader: impl Read) -> Result<HashMap<String, Vec<String>>, String> {
    let reader = BufReader::new(reader);

    let mut rules: HashMap<String, Vec<String>> = HashMap::new();

    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                let (outer, inner) = parse_rule(&x).unwrap();
                for i in inner {
                    if let Some(x) = rules.get_mut(i) {
                        x.push(outer.to_owned());
                    } else {
                        rules.insert(i.to_owned(), vec![outer.to_owned()]);
                    }
                }
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(rules)
}

fn traverse_rules(
    mut visited: HashSet<String>,
    rules: &HashMap<String, Vec<String>>,
    color: &str,
) -> HashSet<String> {
    if let Some(s) = rules.get(color) {
        for outer in s {
            println!("from {:?} to {:?} {:?}", color, outer, visited);
            visited.insert(outer.to_owned());
            visited = traverse_rules(visited, rules, outer);
        }
    }
    //println!("{:?}", count);

    visited
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let rules = read_input(input_file.unwrap()).unwrap();

    let mut visited = HashSet::new();
    visited = traverse_rules(visited, &rules, &String::from("shiny gold"));
    println!("{:?}", visited.len());
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
        assert_eq!(output["bright white"], vec!["light red", "dark orange"]);
        assert_eq!(output["shiny gold"], vec!["bright white", "muted yellow"]);
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
        let mut visited = HashSet::new();
        visited = traverse_rules(visited, &output, &String::from("shiny gold"));
        assert_eq!(visited.len(), 4);
        assert_eq!(visited.contains("bright white"), true);
        assert_eq!(visited.contains("muted yellow"), true);
        assert_eq!(visited.contains("dark orange"), true);
        assert_eq!(visited.contains("light red"), true);
    }

    #[test]
    fn test_parse_rule() {
        let (outer, inners) =
            parse_rule("light red bags contain 1 bright white bag, 2 muted yellow bags.").unwrap();
        assert_eq!(outer, "light red");
        assert_eq!(inners, vec!["bright white", "muted yellow"]);
    }
}
