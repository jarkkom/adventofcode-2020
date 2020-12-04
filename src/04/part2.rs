use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::env;
use std::collections::HashMap;
use regex::Regex;


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
                        //println!("field: {:?}", field);
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

fn is_number_in_range(num: &str, min: i64, max: i64) -> bool {
    let n: i64 = num.parse().unwrap_or_default();
    if n < min || n > max { 
        println!("invalid {:?} not in {}-{}", num, min, max);
        return false;
    }
    return true;
}


fn is_valid_height(hgt: &str) -> bool {
    let hgt_re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    if let Some(hgt_capt) = hgt_re.captures(hgt) {
        if hgt_capt.len() != 3 {
            return false
        };

        let vs = hgt_capt.get(1).map_or("", |m| m.as_str());
        let t = hgt_capt.get(2).map_or("", |m| m.as_str());

        let v: i64 = vs.parse().unwrap_or_default();

        if t == "cm" {
            return v >= 150 && v <= 193;
        }
        if t == "in" {
            return v >= 59 && v <= 76;
        }
    }
    println!("invalid height {:?}", hgt);
    return false;
}

fn is_valid_hcl(color: &str) -> bool {
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    return re.is_match(color);
}

fn is_valid_ecl(color: &str) -> bool {
    let mandatory_fields = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    return mandatory_fields.contains(&color);
}

fn is_valid_pid(pid: &str) -> bool {
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    return re.is_match(pid);
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
            is_valid = is_number_in_range(passport.get("byr").unwrap(), 1920, 2002)
                && is_number_in_range(passport.get("iyr").unwrap(), 2010, 2020)
                && is_number_in_range(passport.get("eyr").unwrap(), 2020, 2030)
                && is_valid_height(passport.get("hgt").unwrap())
                && is_valid_hcl(passport.get("hcl").unwrap())
                && is_valid_ecl(passport.get("ecl").unwrap())
                && is_valid_pid(passport.get("pid").unwrap())
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
    fn test_valid_height() {
        assert_eq!(is_valid_height("173cm"), true);
        assert_eq!(is_valid_height("65in"), true);
    }

    #[test]
    fn test_invalid_height() {
        assert_eq!(is_valid_height("173"), false);
        assert_eq!(is_valid_height("173cc"), false);
        assert_eq!(is_valid_height("in"), false);
    }

    #[test]
    fn test_invalid_height_ranges() {
        assert_eq!(is_valid_height("1in"), false);
        assert_eq!(is_valid_height("100in"), false);
        assert_eq!(is_valid_height("149cm"), false);
        assert_eq!(is_valid_height("194cm"), false);
        assert_eq!(is_valid_height("58in"), false);
        assert_eq!(is_valid_height("77in"), false);
    }

    #[test]
    fn test_valid_hcl() {
        assert_eq!(is_valid_hcl("#123abc"), true);
        assert_eq!(is_valid_hcl("#123abz"), false);
        assert_eq!(is_valid_hcl("123abc"), false);
    }

    #[test]
    fn test_valid_ecl() {
        assert_eq!(is_valid_ecl("brn"), true);
        assert_eq!(is_valid_ecl("wat"), false);
    }

    #[test]
    fn test_valid_pid() {
        assert_eq!(is_valid_pid("000000001"), true);
        assert_eq!(is_valid_pid("0123456789"), false);
    }
}
