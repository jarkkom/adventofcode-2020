use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::env;
use std::collections::HashMap;


fn read_input(filename: &str) -> Result<Vec<HashMap<String, String>>, String> {
    let path = Path::new(filename);
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());

    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    let mut curr_passport = HashMap::new();

    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                if x.is_empty() {
                    passports.push(curr_passport.to_owned());
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
    if curr_passport.len() > 0 {
        passports.push(curr_passport.to_owned());
    }

    return Ok(passports);
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
    return valids;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let passports = read_input(&filename).unwrap();

    println!("valid passports {:?} ", count_valid_passports(passports));
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_valid_passports() {
    }
}
