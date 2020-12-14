use std::collections::HashMap;
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

fn read_input(reader: impl Read) -> Result<Vec<String>, String> {
    let reader = BufReader::new(reader);

    let mut lines: Vec<String> = Vec::new();

    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                lines.push(x.parse().unwrap());
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(lines)
}

#[derive(Debug, PartialEq)]
struct CPU {
    mem: HashMap<i64, i64>,
    mask0: i64,
    mask1: i64,
}

impl CPU {
    fn set_mask(&mut self, mask: &str) {
        self.mask0 = 0;
        self.mask1 = 0;

        for c in mask.chars() {
            self.mask0 <<= 1;
            self.mask1 <<= 1;
            match c {
                '0' => {
                    self.mask0 |= 1;
                },
                '1' => {
                    self.mask1 |= 1;
                },
                _ => {},
            }
        }

        println!("set_mask {} {:36b} {:36b}", mask, self.mask0, self.mask1);
    }

    fn set_mem(&mut self, loc: i64, val: i64) {
        println!("set_mem {} = {} ", loc, val);

        println!("val   {:64b}", val);
        println!("mask0 {:64b}", !self.mask0);
        println!("mask1 {:64b}", self.mask1);
        println!("res   {:64b}", val & (!self.mask0) | self.mask1);

        self.mem.insert(loc, val & !self.mask0 | self.mask1);
    }

    fn execute(&mut self, instr: &str) {
        let mask_re = Regex::new(r"mask = ([01X]+)").unwrap();
        let mem_re = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();

        if mask_re.is_match(instr) {
            let caps = mask_re.captures(instr).unwrap();
            self.set_mask(caps.get(1).unwrap().as_str());
        }

        if mem_re.is_match(instr) {
            let caps = mem_re.captures(instr).unwrap();
            println!("caps {:?}", caps);
            let loc = caps.get(1).unwrap().as_str().parse().unwrap();
            let val = caps.get(2).unwrap().as_str().parse().unwrap();

            self.set_mem(loc, val);
        }
    }

    fn mem_sum(&self) -> i64 {
        self.mem.values().fold(0, |a, m| a + m)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let inputs = read_input(input_file.unwrap()).unwrap();

    let mut cpu = CPU {
        mem: HashMap::new(),
        mask0: 0,
        mask1: 0,
    };

    for i in inputs {
        cpu.execute(&i);
    }

    println!("answer = {}", cpu.mem_sum());

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        let inputs = read_input(input.as_bytes()).unwrap();

        let mut cpu = CPU {
            mem: HashMap::new(),
            mask0: 0,
            mask1: 0,
        };

        cpu.execute(&inputs[0]);
        assert_eq!(cpu.mask0, 0b0000010);
        assert_eq!(cpu.mask1, 0b1000000);

        cpu.execute(&inputs[1]);
        assert_eq!(cpu.mem[&8], 73);

        cpu.execute(&inputs[2]);
        assert_eq!(cpu.mem[&7], 101);

        cpu.execute(&inputs[3]);
        assert_eq!(cpu.mem[&8], 64);

        assert_eq!(cpu.mem_sum(), 165);
    }
}
