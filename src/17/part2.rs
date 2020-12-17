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

#[derive(PartialEq, Debug)]
struct Cubes {
    active: HashSet<i64>,
}

impl Cubes {
    fn get_index(x: i64, y: i64, z: i64, w: i64) -> i64 {
        (x + 512) << 30 | (y + 512) << 20 | (z + 512) << 10 | (w + 512)
    }

    fn is_occupied(&self, x: i64, y: i64, z: i64, w: i64) -> i64 {
        if self.active.contains(&Cubes::get_index(x, y, z, w)) {
            1
        } else {
            0
        }
    }

    fn apply_rules(&self, area: i64) -> Cubes {
        let mut new_active = HashSet::new();

        for x in -area..area + 1 {
            for y in -area..area + 1 {
                for z in -area..area + 1 {
                    for w in -area..area + 1 {
                        let this_occupied = self.is_occupied(x, y, z, w);

                        let mut occupied = 0;
                        for dx in -1..2 {
                            for dy in -1..2 {
                                for dz in -1..2 {
                                    for dw in -1..2 {
                                        if dx | dy | dz | dw == 0 {
                                            continue;
                                        }
                                        occupied +=
                                            self.is_occupied(x + dx, y + dy, z + dz, w + dw);
                                    }
                                }
                            }
                        }

                        if this_occupied == 1 && (occupied == 2 || occupied == 3) {
                            new_active.insert(Cubes::get_index(x, y, z, w));
                        }
                        if this_occupied == 0 && occupied == 3 {
                            new_active.insert(Cubes::get_index(x, y, z, w));
                        }
                    }
                }
            }
        }

        Cubes { active: new_active }
    }

    fn print(&self, z: i64, w: i64, area: i64) {
        for y in -area..area + 1 {
            for x in -area..area + 1 {
                if self.is_occupied(x, y, z, w) == 1 {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
        println!();
    }
}

fn read_input(reader: impl Read) -> Result<Cubes, String> {
    let reader = BufReader::new(reader);

    let mut active = HashSet::new();

    let lines: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let mut iy = -(lines.len() as i64) / 2;

    for line in lines {
        let mut ix = -(line.len() as i64) / 2;
        for c in line.chars() {
            let index = Cubes::get_index(ix, iy, 0, 0);

            if let '#' = c {
                println!("adding active {} {} @ {}", ix, iy, index);
                active.insert(index);
            }
            ix += 1;
        }
        iy += 1;
    }

    Ok(Cubes { active })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let init_state = read_input(input_file.unwrap()).unwrap();

    let mut curr_state = init_state;
    let mut iterations = 0;
    while iterations < 6 {
        let new_state = curr_state.apply_rules(5 + iterations);
        curr_state = new_state;
        iterations += 1;
    }
    println!("iterations {}", iterations);

    println!("occupied {}", curr_state.active.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> String {
        return String::from(
            ".#.
..#
###",
        );
    }

    #[test]
    fn test_read_input() {
        let map = read_input(get_test_data().as_bytes()).unwrap();

        assert_eq!(map.active.len(), 5);

        println!("{:?}", map.active);

        assert_eq!(map.is_occupied(-1, -1, 0, 0), 0);
        assert_eq!(map.is_occupied(0, -1, 0, 0), 1);
        assert_eq!(map.is_occupied(1, -1, 0, 0), 0);

        assert_eq!(map.is_occupied(-1, 0, 0, 0), 0);
        assert_eq!(map.is_occupied(0, 0, 0, 0), 0);
        assert_eq!(map.is_occupied(1, 0, 0, 0), 1);

        assert_eq!(map.is_occupied(-1, 1, 0, 0), 1);
        assert_eq!(map.is_occupied(0, 1, 0, 0), 1);
        assert_eq!(map.is_occupied(1, 1, 0, 0), 1);

        map.print(0, 0, 1);
    }

    #[test]
    fn test_apply_rules() {
        let mut map = read_input(get_test_data().as_bytes()).unwrap();

        for i in 0..6 {
            map = map.apply_rules(2 + i);
        }

        assert_eq!(map.active.len(), 848);
    }
}
