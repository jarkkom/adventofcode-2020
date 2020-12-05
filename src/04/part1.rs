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

fn read_input(reader: impl Read) -> Result<Vec<HashMap<String, String>>, String> {
    let reader = BufReader::new(reader);

    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    let mut curr_passport = HashMap::new();

    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                if x.is_empty() {
                    passports.push(curr_passport);
                    curr_passport = HashMap::new();
                } else {
                    for field in x.split_ascii_whitespace() {
                        let kv: Vec<&str> = field.split(':').collect();
                        curr_passport.insert(kv[0].to_owned(), kv[1].to_owned());
                    }
                }
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }
    if !curr_passport.is_empty() {
        passports.push(curr_passport);
    }

    Ok(passports)
}

fn count_valid_passports(passports: Vec<HashMap<String, String>>) -> i64 {
    let mut valids = 0;
    let mandatory_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for passport in passports.iter() {
        let mut is_valid = true;
        for mf in mandatory_fields.iter() {
            if !passport.contains_key(mf.to_owned()) {
                println!("did not find mandatory {} in {:?}", mf, passport);
                is_valid = false;
                break;
            }
        }

        if is_valid {
            println!("valid {:?} {}", passport, passport.len());
            valids += 1;
        }
    }
    valids
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let passports = read_input(input_file.unwrap()).unwrap();

    println!("valid passports {:?} ", count_valid_passports(passports));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_reader() {
        let test_input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";
        let output = read_input(test_input.as_bytes()).unwrap();
        assert_eq!(output.len(), 1);
        let passport = output.first().unwrap();
        assert_eq!(passport.len(), 8);
        assert_eq!(passport["ecl"], "gry");
        assert_eq!(passport["pid"], "860033327");
        assert_eq!(passport["eyr"], "2020");
        assert_eq!(passport["hcl"], "#fffffd");
        assert_eq!(passport["byr"], "1937");
        assert_eq!(passport["iyr"], "2017");
        assert_eq!(passport["cid"], "147");
        assert_eq!(passport["hgt"], "183cm");
    }

    #[test]
    fn test_count_valid_passports() {}
}
