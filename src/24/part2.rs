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

fn read_input(reader: impl Read) -> Result<Vec<Vec<Move>>, String> {
    let reader = BufReader::new(reader);

    let mut moves: Vec<Vec<Move>> = Vec::new();

    let mut lines_iter = reader.lines();

    while let Some(line) = lines_iter.next() {
        match line {
            Ok(x) => {
                moves.push(parse_moves(x));
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    Ok(moves)
}

fn parse_moves(s: String) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    let mut i = s.chars();

    while let Some(c) = i.next() {
        match c {
            'w' => moves.push((-1, 0, 1)),
            'e' => moves.push((1, 0, -1)),
            's' => match i.next() {
                Some('w') => moves.push((-1, 1, 0)),
                Some('e') => moves.push((0, 1, -1)),
                _ => panic!("unknown move s{:?}", c),
            },
            'n' => match i.next() {
                Some('w') => moves.push((0, -1, 1)),
                Some('e') => moves.push((1, -1, 0)),
                _ => panic!("unknown move n{:?}", c),
            },
            _ => panic!("unknown move {:?}", c),
        }
    }

    moves
}

fn get_neighbors(m: &Move) -> Vec<Move> {
    let offsets = vec![
        (-1, 0, 1),
        (1, 0, -1),
        (-1, 1, 0),
        (0, 1, -1),
        (0, -1, 1),
        (1, -1, 0),
    ];

    let mut n = Vec::new();
    for o in offsets {
        n.push((m.0 + o.0, m.1 + o.1, m.2 + o.2));
    }
    n
}

fn run_day(blacks: &HashSet<Move>) -> HashSet<Move> {
    let mut res_blacks: HashSet<Move> = HashSet::new();

    // all potential tiles to consider
    let mut all_tiles: HashSet<Move> = HashSet::new();

    for b in blacks {
        all_tiles.insert(b.clone());
        for n in get_neighbors(b) {
            all_tiles.insert(n.clone());
        }
    }

    for t in all_tiles {
        let neighbors = get_neighbors(&t);

        let mut black_neighbors = 0;
        for n in neighbors {
            if blacks.contains(&n) {
                black_neighbors += 1;
            }
        }

        //println!("checking {:?} is_black {:?} black_neigbors {:?}", t, blacks.contains(&t), black_neighbors);

        if blacks.contains(&t) {
            // Any black tile with zero or more than 2 black tiles immediately adjacent to it is flipped to white.
            if black_neighbors == 1 || black_neighbors == 2 {
                res_blacks.insert(t);
            }
        } else {
            // Any white tile with exactly 2 black tiles immediately adjacent to it is flipped to black.
            if black_neighbors == 2 {
                res_blacks.insert(t);
            }
        }
    }

    res_blacks
}

type Move = (i64, i64, i64);

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename).unwrap();

    let moves_list = read_input(input_file).unwrap();

    println!("moves {:?}", moves_list);

    let mut flipped: HashSet<Move> = HashSet::new();

    for moves in moves_list {
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        for m in moves {
            x += m.0;
            y += m.1;
            z += m.2;
        }

        if flipped.contains(&(x, y, z)) {
            flipped.remove(&(x, y, z));
        } else {
            flipped.insert((x, y, z));
        }
    }

    let mut day = 0;
    while day < 100 {
        flipped = run_day(&flipped);
        println!("Day {}: {}", day, flipped.len());

        day += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_moves() {
        let test_input = get_test_input();

        let moves = read_input(test_input.as_bytes()).unwrap();
        println!("moves {:?}", moves);
    }

    #[test]
    fn test_run_day() {
        let moves_list = read_input(get_test_input().as_bytes()).unwrap();
        let mut black_tiles: HashSet<Move> = HashSet::new();
        for moves in moves_list {
            let mut x = 0;
            let mut y = 0;
            let mut z = 0;
            for m in moves {
                x += m.0;
                y += m.1;
                z += m.2;
            }
            if black_tiles.contains(&(x, y, z)) {
                black_tiles.remove(&(x, y, z));
            } else {
                black_tiles.insert((x, y, z));
            }
        }

        assert_eq!(black_tiles.len(), 10);

        black_tiles = run_day(&black_tiles);
        assert_eq!(black_tiles.len(), 15);

        black_tiles = run_day(&black_tiles);
        assert_eq!(black_tiles.len(), 12);

        black_tiles = run_day(&black_tiles);
        assert_eq!(black_tiles.len(), 25);
    }

    fn get_test_input() -> String {
        String::from(
            "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew",
        )
    }
}
