use std::collections::VecDeque;
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
enum ParserItem {
    Mul,
    Add,
    Number(i64),
}

fn collapse_stack(mut stack: VecDeque<ParserItem>) -> VecDeque<ParserItem> {
    let rhitem = stack.pop_front().unwrap();
    let op = stack.pop_front().unwrap();
    let lhitem = stack.pop_front().unwrap();

    if let ParserItem::Number(rhval) = rhitem {
        match lhitem {
            ParserItem::Number(lhval) => match op {
                ParserItem::Add => {
                    stack.push_back(ParserItem::Number(lhval + rhval));
                }
                ParserItem::Mul => {
                    stack.push_back(ParserItem::Number(lhval * rhval));
                }
                _ => {
                    panic!("op was not op {:?}", op);
                }
            },
            _ => {
                panic!("lhitem was not val {:?}", lhitem);
            }
        }
    }

    stack
}

fn parse_expression(iter: &[char]) -> (usize, ParserItem) {
    let mut stack: VecDeque<ParserItem> = VecDeque::new();

    let mut i = 0;

    while i < iter.len() {
        let c = iter[i];
        match c {
            '0'..='9' => {
                println!("{} part of number", c);
                // parse number
                let n = c.to_digit(10).unwrap() as i64;

                stack.push_back(ParserItem::Number(n));

                if stack.len() > 1 {
                    stack = collapse_stack(stack);
                }
            }
            '+' => {
                println!("{} addition", c);
                stack.push_back(ParserItem::Add);
            }
            '*' => {
                println!("{} multiply", c);
                stack.push_back(ParserItem::Mul);
            }
            '(' => {
                println!("{} subexpression", c);
                let (skip, item) = parse_expression(&iter[i + 1..]);
                i += skip + 1;

                stack.push_back(item);

                if stack.len() > 1 {
                    stack = collapse_stack(stack);
                }
            }
            ')' => {
                println!("{} end subexpression", c);
                break;
            }
            ' ' => {}
            _ => {
                panic!("invalid {:?}", c);
            }
        }
        println!("stack {:?} i {}", stack, i);
        i += 1;
    }

    println!("return stack {:?}", stack);
    (i, stack.pop_front().unwrap())
}

fn read_input(reader: impl Read) -> Result<Vec<ParserItem>, String> {
    let reader = BufReader::new(reader);

    let mut expressions: Vec<ParserItem> = Vec::new();

    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                let s: Vec<char> = x.chars().collect();
                let (_, item) = parse_expression(&s);
                expressions.push(item);
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(expressions)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let expressions = read_input(input_file.unwrap()).unwrap();

    println!("expressions = {:?}", expressions);

    let sum: i64 = expressions
        .iter()
        .map(|x| match x {
            ParserItem::Number(n) => n,
            _ => &0,
        })
        .sum();

    println!("sum = {:?}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_reader() {
        let test_input = "1 + 2 * 3 + 4 * 5 + 6
1 + (2 * 3) + (4 * (5 + 6))
2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let output = read_input(test_input.as_bytes()).unwrap();

        println!("output = {:?}", output);
        assert_eq!(output.len(), 6);

        assert_eq!(output[0], ParserItem::Number(71));
        assert_eq!(output[1], ParserItem::Number(51));
        assert_eq!(output[2], ParserItem::Number(26));
        assert_eq!(output[3], ParserItem::Number(437));
        assert_eq!(output[4], ParserItem::Number(12240));
        assert_eq!(output[5], ParserItem::Number(13632));
    }
}
