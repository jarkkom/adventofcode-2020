use regex::Regex;
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

#[derive(PartialEq, Debug)]
struct Rule {
    name: String,
    range1: (i64, i64),
    range2: (i64, i64),
}

fn parse_rule(rule: &str) -> Option<Rule> {
    let rule_re = Regex::new(r"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    if let Some(rule_capt) = rule_re.captures(rule) {
        if rule_capt.len() == 6 {
            println!("rule_capt {:?}", rule_capt);

            return Some(Rule {
                name: rule_capt[1].to_owned(),
                range1: (rule_capt[2].parse().unwrap(), rule_capt[3].parse().unwrap()),
                range2: (rule_capt[4].parse().unwrap(), rule_capt[5].parse().unwrap()),
            });
        };

        return None;
    }
    None
}

fn read_rules(reader: impl Read) -> Result<Vec<Rule>, String> {
    let reader = BufReader::new(reader);

    let mut rules: Vec<Rule> = Vec::new();

    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                if x.is_empty() {
                    break;
                }

                let rule = parse_rule(&x).unwrap();
                rules.push(rule);
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(rules)
}

fn read_tickets(reader: impl Read) -> Result<Vec<Vec<i64>>, String> {
    let reader = BufReader::new(reader);

    let mut iter = reader.lines();
    iter.next();

    let mut tickets: Vec<Vec<i64>> = Vec::new();

    for line_iter in iter {
        match line_iter {
            Ok(x) => {
                if x.is_empty() {
                    break;
                }

                tickets.push(x.split(',').map(|x| x.parse().unwrap()).collect());

                println!("{:?}", x);
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(tickets)
}

fn read_input(reader: impl Read) -> Result<(Vec<Rule>, Vec<i64>, Vec<Vec<i64>>), String> {
    let reader = BufReader::new(reader);

    let mut rules: Vec<Rule> = Vec::new();

    let mut lines_iter = reader.lines();

    loop {
        let line: io::Result<String> = lines_iter.next().unwrap();
        match line {
            Ok(x) => {
                if x.is_empty() {
                    break;
                }

                let rule = parse_rule(&x).unwrap();
                rules.push(rule);
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    let _x = lines_iter.next();

    let mut own_tickets: Vec<Vec<i64>> = Vec::new();
    loop {
        let line: io::Result<String> = lines_iter.next().unwrap();
        match line {
            Ok(x) => {
                if x.is_empty() {
                    break;
                }

                own_tickets.push(x.split(',').map(|x| x.parse().unwrap()).collect());

                println!("{:?}", x);
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    let _x = lines_iter.next();
    let mut tickets: Vec<Vec<i64>> = Vec::new();
    loop {
        if let Some(line) = lines_iter.next() {
            match line {
                Ok(x) => {
                    if x.is_empty() {
                        break;
                    }

                    tickets.push(x.split(',').map(|x| x.parse().unwrap()).collect());

                    println!("{:?}", x);
                }
                Err(x) => {
                    return Err(format!("cannot read input: {:?}", x));
                }
            }
        } else {
            break;
        }
    }
    Ok((rules, own_tickets[0].to_vec(), tickets))
}

fn validate_ticket(ticket: &Vec<i64>, rules: &Vec<Rule>) -> Option<i64> {
    for &t in ticket {
        let mut valid_rules = 0;
        for r in rules {
            println!("comparing {:?} to {:?}", t, r);
            if (t >= r.range1.0 && t <= r.range1.1) || (t >= r.range2.0 && t <= r.range2.1) {
                valid_rules += 1;
            }
        }
        println!("valid rules for {} {}", t, valid_rules);
        if valid_rules == 0 {
            return Some(t);
        }
    }
    None
}

fn calculate_error_rate(invalid_values: Vec<Option<i64>>) -> i64 {
    invalid_values.iter().map(|x| x.unwrap_or(0)).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename).unwrap();

    let (rules, _my_ticket, tickets) = read_input(input_file).unwrap();

    println!("tickets {:?}", tickets);

    let validated_tickets: Vec<Option<i64>> = tickets
        .iter()
        .map(|t| validate_ticket(&t, &rules))
        .collect();
    println!("validated_tickets {:?}", validated_tickets);

    println!("answer {:?}", calculate_error_rate(validated_tickets));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_rules() {
        let test_input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"
            .as_bytes();

        let rules = read_rules(test_input).unwrap();
        println!("{:?}", rules);
        assert_eq!(rules.len(), 3);
        assert_eq!(
            rules[0],
            Rule {
                name: String::from("class"),
                range1: (1, 3),
                range2: (5, 7),
            }
        );
        assert_eq!(
            rules[1],
            Rule {
                name: String::from("row"),
                range1: (6, 11),
                range2: (33, 44),
            }
        );
        assert_eq!(
            rules[2],
            Rule {
                name: String::from("seat"),
                range1: (13, 40),
                range2: (45, 50),
            }
        );
    }

    #[test]
    fn test_read_tickets() {
        let test_input_1 = "your ticket:
7,1,14

";
        let test_input_2 = "nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

        let actual_1 = read_tickets(test_input_1.as_bytes()).unwrap();
        println!("{:?}", actual_1);
        assert_eq!(actual_1.len(), 1);
        assert_eq!(actual_1[0], vec![7, 1, 14]);

        let actual_2 = read_tickets(test_input_2.as_bytes()).unwrap();
        println!("{:?}", actual_2);
        assert_eq!(actual_2.len(), 4);
        assert_eq!(actual_2[0], vec![7, 3, 47]);
        assert_eq!(actual_2[1], vec![40, 4, 50]);
        assert_eq!(actual_2[2], vec![55, 2, 20]);
        assert_eq!(actual_2[3], vec![38, 6, 12]);
    }

    #[test]
    fn test_validate_tickets() {
        let test_rule_input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50
";
        let test_input_tickets = "nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

        let rules = read_rules(test_rule_input.as_bytes()).unwrap();
        let tickets = read_tickets(test_input_tickets.as_bytes()).unwrap();

        assert_eq!(validate_ticket(&tickets[0], &rules), None);
        assert_eq!(validate_ticket(&tickets[1], &rules), Some(4));
        assert_eq!(validate_ticket(&tickets[2], &rules), Some(55));
        assert_eq!(validate_ticket(&tickets[3], &rules), Some(12));

        let validated_tickets = tickets.iter().map(|x| validate_ticket(x, &rules)).collect();

        assert_eq!(calculate_error_rate(validated_tickets), 71);
    }
}
