use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

struct Map {
    width: usize,
    height: usize,
    trees: Vec<bool>,
}

fn read_input(filename: &str) -> Result<Map, String> {
    let path = Path::new(filename);
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());

    let mut trees = Vec::new();
    let mut max_width = 0;
    let mut height = 0;
    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                let mut line_width = 0;
                for c in x.chars() {
                    if c == '.' || c == '#' {
                        line_width += 1;
                        trees.push(c == '#');
                    }
                }
                if line_width > max_width {
                    max_width = line_width;
                }
                height += 1;
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(Map {
        width: max_width,
        height,
        trees,
    })
}

fn count_trees(map: Map) -> i64 {
    let mut trees = 0;

    let mut y = 0;
    let mut x = 0;
    while y < map.height {
        if map.trees[y * map.width + x] {
            trees += 1;
        }
        x = (x + 3) % map.width;
        y += 1;
    }
    trees
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let map = read_input(&filename).unwrap();

    println!("map {:?} {:?} {:?}", map.width, map.height, map.trees);

    println!("trees {:?} ", count_trees(map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_trees() {
        let test_map = Map {
            width: 11,
            height: 11,
            trees: vec![
                false, false, true, true, false, false, false, false, false, false, false, true,
                false, false, false, true, false, false, false, true, false, false, false, true,
                false, false, false, false, true, false, false, true, false, false, false, true,
                false, true, false, false, false, true, false, true, false, true, false, false,
                false, true, true, false, false, true, false, false, false, true, false, true,
                true, false, false, false, false, false, false, true, false, true, false, true,
                false, false, false, false, true, false, true, false, false, false, false, false,
                false, false, false, true, true, false, true, true, false, false, false, true,
                false, false, false, true, false, false, false, true, true, false, false, false,
                false, true, false, true, false, false, true, false, false, false, true, false,
                true,
            ],
        };

        assert_eq!(count_trees(test_map), 7);
    }
}
