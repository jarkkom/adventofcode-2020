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

fn parse_allergens(rule: &str) -> Option<(Vec<String>, Vec<String>)> {
    let rule_re = Regex::new(r"^(.*) \(contains (.*)\)$").unwrap();
    if let Some(rule_capt) = rule_re.captures(rule) {
        if rule_capt.len() != 3 {
            return None;
        };

        let ingredients_capt = rule_capt.get(1).map_or("", |m| m.as_str());
        let allergens_capt = rule_capt.get(2).map_or("", |m| m.as_str());

        let ingredients = ingredients_capt.split(' ').map(|i| i.to_owned()).collect();
        let allergens = allergens_capt
            .split(',')
            .map(|i| i.trim().to_owned())
            .collect();

        return Some((ingredients, allergens));
    }
    None
}

fn read_input(reader: impl Read) -> Result<Vec<(Vec<String>, Vec<String>)>, String> {
    let reader = BufReader::new(reader);

    //let mut result: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    let mut result: Vec<(Vec<String>, Vec<String>)> = Vec::new();

    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                if !x.is_empty() {
                    let (ingredients, allergens) = parse_allergens(&x).unwrap();
                    println!("{:?} {:?}", ingredients, allergens);

                    result.push((ingredients, allergens));
                }
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(result)
}

fn map_candidates(foods: Vec<(Vec<String>, Vec<String>)>) -> HashMap<String, Vec<Vec<String>>> {
    let mut candidates: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    for food in foods {
        for allergen in food.1 {
            candidates.entry(allergen).or_default().push(food.0.clone());
        }
    }

    candidates
}

fn find_allergens(candidates: &HashMap<String, Vec<Vec<String>>>) -> HashMap<String, String> {
    let mut allergens: HashMap<String, String> = HashMap::new();

    loop {
        candidates.iter().for_each(|(a, i)| {
            let mut temp_set: HashSet<&String> = i.get(0).unwrap().iter().collect();

            i.iter().skip(1).for_each(|i| {
                let other_set: HashSet<&String> = i.iter().collect();
                temp_set = temp_set.intersection(&other_set).copied().collect();
            });

            // and remove ingredients that we've always found
            for found_ingredient in allergens.keys() {
                temp_set.remove(found_ingredient);
            }

            // only one ingredient can be allegren
            if temp_set.len() == 1 {
                // remove it from other sets
                allergens.insert(
                    temp_set
                        .iter()
                        .take(1)
                        .next()
                        .unwrap()
                        .to_owned()
                        .to_string(),
                    a.clone(),
                );
            }
        });

        if allergens.len() == candidates.len() {
            break;
        }
    };

    allergens
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let foods = read_input(input_file.unwrap()).unwrap();

    println!("foods {:?}", foods);

    let candidates = map_candidates(foods.clone());

    println!("candidates {:?}", candidates);

    let allergens = find_allergens(&candidates);

    println!("allergens {:?}", allergens);

    let answer = foods.iter().fold(0, |a, f| {
        a + f.0.iter().filter(|&i| !allergens.contains_key(i)).count()
    });
    println!("answer {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_reader() {
        let foods = read_input(get_test_input().as_bytes()).unwrap();
        println!("{:?}", foods);

        let candidates = map_candidates(foods);

        assert_eq!(candidates.len(), 3);
        assert_eq!(
            candidates["fish"],
            vec![
                vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"],
                vec!["sqjhc", "mxmxvkd", "sbzzf"]
            ]
        );
        assert_eq!(candidates["soy"], vec![vec!["sqjhc", "fvjkl"]]);
        assert_eq!(
            candidates["dairy"],
            vec![
                vec!["mxmxvkd", "kfcds", "sqjhc", "nhms"],
                vec!["trh", "fvjkl", "sbzzf", "mxmxvkd"]
            ]
        );
    }

    #[test]
    fn test_find_allergens() {
        let foods = read_input(get_test_input().as_bytes()).unwrap();
        println!("{:?}", foods);

        let candidates = map_candidates(foods.clone());

        let allergens = find_allergens(&candidates);

        println!("allergens {:?}", allergens);

        assert_eq!(allergens["mxmxvkd"], "dairy");
        assert_eq!(allergens["sqjhc"], "fish");
        assert_eq!(allergens["fvjkl"], "soy");

        let answer = foods.iter().fold(0, |a, f| {
            a + f.0.iter().filter(|&i| !allergens.contains_key(i)).count()
        });
        assert_eq!(answer, 5);
    }

    fn get_test_input() -> String {
        String::from(
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)",
        )
    }
}
