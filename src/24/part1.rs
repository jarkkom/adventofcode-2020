use std::collections::HashSet;
use std::collections::VecDeque;
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

    println!("count = {}", flipped.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_rules() {
        let test_input = get_test_input();

        let moves = read_input(test_input.as_bytes()).unwrap();
        println!("moves {:?}", moves);
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
