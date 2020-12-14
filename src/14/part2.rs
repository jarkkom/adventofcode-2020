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
    maskx: i64,
}

impl CPU {
    fn set_mask(&mut self, mask: &str) {
        self.mask0 = 0;
        self.mask1 = 0;
        self.maskx = 0;

        for c in mask.chars() {
            self.mask0 <<= 1;
            self.mask1 <<= 1;
            self.maskx <<= 1;
            match c {
                '0' => {
                    self.mask0 |= 1;
                }
                '1' => {
                    self.mask1 |= 1;
                }
                'X' => {
                    self.maskx |= 1;
                }
                _ => {}
            }
        }

        println!("set_mask {} {:36b} {:36b}", mask, self.mask0, self.mask1);
    }

    fn set_mem(&mut self, loc: i64, val: i64) {
        let mut addrs = vec![loc | self.mask1];

        for x in 0..36 {
            if (self.maskx & (1 << x)) != 0 {
                let mut newaddrs = vec![];
                for a in &addrs {
                    newaddrs.push(a & !(1 << x));
                    newaddrs.push(a & !(1 << x) | 1 << x);
                }
                addrs = newaddrs;
            }
        }
        for a in addrs {
            self.mem.insert(a, val);
        }
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
            println!("executing {:?}", instr);
            let loc = caps.get(1).unwrap().as_str().parse().unwrap();
            let val = caps.get(2).unwrap().as_str().parse().unwrap();

            self.set_mem(loc, val);
        }
    }

    fn mem_sum(&self) -> i64 {
        self.mem.values().sum()
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
        maskx: 0,
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
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let inputs = read_input(input.as_bytes()).unwrap();

        let mut cpu = CPU {
            mem: HashMap::new(),
            mask0: 0,
            mask1: 0,
            maskx: 0,
        };

        cpu.execute(&inputs[0]);
        println!("{:36b}", cpu.mask0);
        println!("{:36b}", cpu.mask1);
        println!("{:36b}", cpu.maskx);

        assert_eq!(cpu.mask0, 0b1111_1111_1111_1111_1111_1111_1111_1100_1100);
        assert_eq!(cpu.mask1, 0b0001_0010);
        assert_eq!(cpu.maskx, 0b0010_0001);

        cpu.execute(&inputs[1]);

        println!("{:?}", cpu.mem);
        assert_eq!(cpu.mem[&26], 100);
        assert_eq!(cpu.mem[&27], 100);
        assert_eq!(cpu.mem[&58], 100);
        assert_eq!(cpu.mem[&59], 100);

        cpu.execute(&inputs[2]);

        cpu.execute(&inputs[3]);

        assert_eq!(cpu.mem[&16], 1);
        assert_eq!(cpu.mem[&17], 1);
        assert_eq!(cpu.mem[&18], 1);
        assert_eq!(cpu.mem[&19], 1);
        assert_eq!(cpu.mem[&24], 1);
        assert_eq!(cpu.mem[&25], 1);
        assert_eq!(cpu.mem[&26], 1);
        assert_eq!(cpu.mem[&27], 1);

        assert_eq!(cpu.mem_sum(), 208);

        /*
                cpu.execute("mask = 00000X0X0000000000000000000000X1001X");
                let before = std::time::Instant::now();
                cpu.execute(&inputs[3]);
                let after = std::time::Instant::now();
                println!("{:?}", after - before);
        */
    }
}
