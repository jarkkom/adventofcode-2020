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

    return Ok(Map {
        width: max_width,
        height: height,
        trees: trees,
    });
}

fn count_trees(stride_x: usize, stride_y: usize, map: &Map) -> i64 {
    let mut trees = 0;

    let mut y = 0;
    let mut x = 0;
    while y < map.height {
        if map.trees[y * map.width + x] {
            trees += 1;
        }
        x = (x + stride_x) % map.width;
        y += stride_y;
    }
    return trees;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let map = read_input(&filename).unwrap();

    let t1 = count_trees(1, 1, &map);
    let t2 = count_trees(3, 1, &map);
    let t3 = count_trees(5, 1, &map);
    let t4 = count_trees(7, 1, &map);
    let t5 = count_trees(1, 2, &map);
    println!(
        "{} * {} * {} * {} * {} = {}",
        t1,
        t2,
        t3,
        t4,
        t5,
        t1 * t2 * t3 * t4 * t5
    );
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

        assert_eq!(count_trees(1, 1, &test_map), 2);
        assert_eq!(count_trees(3, 1, &test_map), 7);
        assert_eq!(count_trees(5, 1, &test_map), 3);
        assert_eq!(count_trees(7, 1, &test_map), 4);
        assert_eq!(count_trees(1, 2, &test_map), 2);
    }
}
