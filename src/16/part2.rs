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

type Ticket = Vec<i64>;

#[derive(PartialEq, Hash, Eq, Debug)]
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

fn read_input(reader: impl Read) -> Result<(Vec<Rule>, Ticket, Vec<Ticket>), String> {
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
    for line in lines_iter {
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
    }
    Ok((rules, own_tickets[0].to_vec(), tickets))
}

fn validate_ticket(ticket: &Ticket, rules: &[Rule]) -> Option<i64> {
    for &t in ticket {
        let mut valid_rules = 0;
        for r in rules {
            //println!("comparing {:?} to {:?}", t, r);
            if (t >= r.range1.0 && t <= r.range1.1) || (t >= r.range2.0 && t <= r.range2.1) {
                valid_rules += 1;
            }
        }
        //println!("valid rules for {} {}", t, valid_rules);
        if valid_rules == 0 {
            return Some(t);
        }
    }
    None
}

fn find_rule_columns(rules: Vec<Rule>, tickets: Vec<&Ticket>) -> HashMap<String, usize> {
    let mut valid_columns_per_rule: HashMap<&Rule, Vec<usize>> = HashMap::new();

    let num_columns = tickets[0].len();

    rules.iter().for_each(|r| {
        let mut valid_columns = Vec::new();
        // check all ticket columns
        for i in 0..num_columns {
            // if all values satisfy this rule, add column to the list
            if tickets.iter().all(|ticket| {
                let t = ticket[i];
                (t >= r.range1.0 && t <= r.range1.1) || (t >= r.range2.0 && t <= r.range2.1)
            }) {
                valid_columns.push(i);
            }
        }
        println!("rule {:?} has valid columns {:?}", r, valid_columns);
        valid_columns_per_rule.insert(&r, valid_columns);
    });

    let mut rule_to_column: HashMap<String, usize> = HashMap::new();

    // go through rules and find rule with least valid columns
    while let Some(smallest) = valid_columns_per_rule
        .iter()
        .filter(|(_k, v)| !v.is_empty())
        .min_by_key(|(_k, v)| v.len())
    {
        println!("smallest rule {:?}", smallest);
        let smallest_column = smallest.1[0];

        // and map that rule to column id
        rule_to_column.insert(smallest.0.name.to_owned(), smallest_column);

        // remove this rule from the list
        valid_columns_per_rule
            .values_mut()
            .filter(|r| !r.is_empty())
            .for_each(|r| {
                let column_to_remove = r
                    .iter()
                    .enumerate()
                    .find(|(_i, &v)| v == smallest_column)
                    .map(|(i, _v)| i);
                if let Some(x) = column_to_remove {
                    r.remove(x);
                }
            });
    }

    rule_to_column
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename).unwrap();

    let (rules, my_ticket, mut tickets) = read_input(input_file).unwrap();

    tickets.push(my_ticket.clone());

    println!("tickets {:?}", tickets);

    let validated_tickets: Vec<&Ticket> = tickets
        .iter()
        .filter(|t| validate_ticket(t, &rules).is_none())
        .collect();

    println!("validated_tickets {:?}", validated_tickets);

    let mapping = find_rule_columns(rules, validated_tickets);

    println!("my_ticket {:?}", my_ticket);

    let answer: i64 = mapping
        .iter()
        .filter(|(k, _v)| k.starts_with("departure"))
        .map(|(_k, &v)| {
            println!("{:?} {:?}", _k, v);
            my_ticket[v]
        })
        .product();

    println!("answer {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_tickets() {
        let test_input = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

        let (rules, my_ticket, mut tickets) = read_input(test_input.as_bytes()).unwrap();

        assert_eq!(rules.len(), 3);
        assert_eq!(
            rules[0],
            Rule {
                name: String::from("class"),
                range1: (0, 1),
                range2: (4, 19),
            }
        );
        assert_eq!(
            rules[1],
            Rule {
                name: String::from("row"),
                range1: (0, 5),
                range2: (8, 19),
            }
        );
        assert_eq!(
            rules[2],
            Rule {
                name: String::from("seat"),
                range1: (0, 13),
                range2: (16, 19),
            }
        );

        assert_eq!(my_ticket, vec![11, 12, 13]);

        assert_eq!(tickets.len(), 3);
        assert_eq!(tickets[0], vec![3, 9, 18]);
        assert_eq!(tickets[1], vec![15, 1, 5]);
        assert_eq!(tickets[2], vec![5, 14, 9]);

        tickets.push(my_ticket.clone());

        let validated_tickets: Vec<&Vec<i64>> = tickets
            .iter()
            .filter(|t| validate_ticket(&t, &rules).is_none())
            .collect();

        let mapping = find_rule_columns(rules, validated_tickets);
        println!("{:?}", mapping);

        assert_eq!(*mapping.get("class").unwrap(), 1usize);
        assert_eq!(*mapping.get("row").unwrap(), 0usize);
        assert_eq!(*mapping.get("seat").unwrap(), 2usize);

        let answer: i64 = mapping
            .iter()
            .map(|(_k, &v)| {
                println!("{:?} {:?}", _k, v);
                my_ticket[v]
            })
            .product();

        assert_eq!(answer, 11 * 12 * 13);
    }
}
