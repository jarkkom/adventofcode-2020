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
struct Opcode {
    instr: String,
    oper: i64,
}

#[derive(Debug, PartialEq)]
struct CPU {
    ip: usize,
    inst: Vec<Opcode>,
    acc: i64,
    debug: bool,
}

fn parse_opcode(s: &str) -> Option<Opcode> {
    let mut i = s.split_whitespace();

    Some(Opcode {
        instr: i.next().unwrap().to_owned(),
        oper: i.next().unwrap().parse().unwrap(),
    })
}

fn read_input(reader: impl Read) -> Result<Vec<Opcode>, String> {
    let reader = BufReader::new(reader);

    let mut opcodes: Vec<Opcode> = Vec::new();

    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                let opcode = parse_opcode(&x).unwrap();
                opcodes.push(opcode);
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(opcodes)
}

impl CPU {
    // return next instruction to exec
    fn execute(&mut self) -> usize {
        let instr = &self.inst[self.ip];

        if self.debug {
            println!(
                "ip {} acc {} opcode {} oper {}",
                self.ip, self.acc, instr.instr, instr.oper,
            );
        }

        match instr.instr.as_str() {
            "nop" => {
                self.ip += 1;
            }
            "acc" => {
                self.acc += instr.oper;
                self.ip += 1;
            }
            "jmp" => {
                self.ip = (self.ip as i64 + instr.oper) as usize;
            }
            _ => {
                panic!("unknown instruction");
            }
        }
        return self.ip;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let opcodes = read_input(input_file.unwrap()).unwrap();
    println!("{:?}", opcodes);

    let mut cpu = CPU {
        ip: 0,
        inst: opcodes,
        acc: 0,
        debug: true,
    };

    let mut executed: Vec<usize> = vec![];
    loop {
        executed.push(cpu.ip);
        let next_ip = cpu.execute();

        if executed.contains(&next_ip) {
            break;
        }
    }

    println!("acc {:?}", cpu.acc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_reader() {
        let test_input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let output = read_input(test_input.as_bytes()).unwrap();
        println!("{:?}", output);

        let expected = vec![
            Opcode {
                instr: String::from("nop"),
                oper: 0,
            },
            Opcode {
                instr: String::from("acc"),
                oper: 1,
            },
            Opcode {
                instr: String::from("jmp"),
                oper: 4,
            },
            Opcode {
                instr: String::from("acc"),
                oper: 3,
            },
            Opcode {
                instr: String::from("jmp"),
                oper: -3,
            },
            Opcode {
                instr: String::from("acc"),
                oper: -99,
            },
            Opcode {
                instr: String::from("acc"),
                oper: 1,
            },
            Opcode {
                instr: String::from("jmp"),
                oper: -4,
            },
            Opcode {
                instr: String::from("acc"),
                oper: 6,
            },
        ];
        assert_eq!(output, expected);
    }

    #[test]
    fn test_execute() {
        let test_input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let opcodes = read_input(test_input.as_bytes()).unwrap();

        let mut cpu = CPU {
            ip: 0,
            inst: opcodes,
            acc: 0,
            debug: true,
        };

        /*
        0 nop +0  | 1
        1 acc +1  | 2, 8(!)
        2 jmp +4  | 3
        3 acc +3  | 6
        4 jmp -3  | 7
        5 acc -99 |
        6 acc +1  | 4
        7 jmp -4  | 5
        8 acc +6  |
        */

        assert_eq!(cpu.ip, 0);
        assert_eq!(cpu.execute(), 1);
        assert_eq!(cpu.execute(), 2);
        assert_eq!(cpu.execute(), 6);
        assert_eq!(cpu.execute(), 7);
        assert_eq!(cpu.execute(), 3);
        assert_eq!(cpu.execute(), 4);
        assert_eq!(cpu.execute(), 1);

        println!("{:?}", cpu.acc);

        assert_eq!(cpu.acc, 5);
    }

    #[test]
    fn test_parse_rule() {
        let opcode1 = parse_opcode("someinstr +12345").unwrap();
        assert_eq!(
            opcode1,
            Opcode {
                instr: String::from("someinstr"),
                oper: 12345
            }
        );
        let opcode2 = parse_opcode("another -12345").unwrap();
        assert_eq!(
            opcode2,
            Opcode {
                instr: String::from("another"),
                oper: -12345
            }
        );
    }
}
