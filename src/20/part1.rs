use regex::Regex;
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

fn read_input(reader: impl Read) -> Result<Vec<Tile>, String> {
    let reader = BufReader::new(reader);

    let mut tiles = Vec::new();

    let mut current_tile = Tile::new();

    let id_regex = Regex::new(r"Tile (\d+):").unwrap();

    for line_iter in reader.lines() {
        match line_iter {
            Ok(x) => {
                if id_regex.is_match(&x) {
                    current_tile = Tile::new();

                    if let Some(captures) = id_regex.captures(&x) {
                        current_tile.id = captures[1].parse().unwrap();
                    }
                } else if x.is_empty() {
                    current_tile.calculcate_edges();
                    tiles.push(current_tile);
                    current_tile = Tile::new();
                } else {
                    for c in x.chars() {
                        current_tile.pixels.push(c);
                    }
                }
            }
            Err(x) => {
                return Err(format!("cannot read input: {:?}", x));
            }
        }
    }

    if current_tile.id > 0 {
        current_tile.calculcate_edges();
        tiles.push(current_tile);
    }

    Ok(tiles)
}

#[derive(Debug)]
struct Tile {
    id: i64,
    pixels: Vec<char>,
    edges: Vec<String>,
}

impl Tile {
    fn new() -> Tile {
        Tile {
            id: 0,
            pixels: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn calculcate_edges(&mut self) {
        let mut top_edge = String::with_capacity(10);
        for x in 0..10 {
            top_edge.push(self.pixels[x]);
        }
        self.edges.push(top_edge);

        let mut right_edge = String::with_capacity(10);
        for y in 0..10 {
            right_edge.push(self.pixels[9 + y * 10]);
        }
        self.edges.push(right_edge);

        let mut bottom_edge = String::with_capacity(10);
        for x in 0..10 {
            bottom_edge.push(self.pixels[10 * 9 + x]);
        }
        self.edges.push(bottom_edge);

        let mut left_edge = String::with_capacity(10);
        for y in 0..10 {
            left_edge.push(self.pixels[10 * y]);
        }
        self.edges.push(left_edge);
    }

    fn find_potential_neighbours(&self, tiles: &[Tile]) -> Vec<i64> {
        let mut potential_neighbours = Vec::new();

        for t in tiles.iter() {
            if self.id == t.id {
                continue;
            }

            for e in &self.edges {
                let reverse_e: String = e.chars().rev().collect();

                if t.edges.contains(&e) || t.edges.contains(&reverse_e) {
                    potential_neighbours.push(t.id);
                }
            }
        }

        potential_neighbours
    }

    fn is_corner_tile(&self, tiles: &[Tile]) -> bool {
        self.find_potential_neighbours(tiles).len() == 2
    }

    fn rotate(&mut self) {
        self.edges.rotate_right(1);
    }

    fn flip(&mut self) {
        let mut new_edges = Vec::with_capacity(4);

        new_edges.push(self.edges[0].chars().rev().collect());
        new_edges.push(self.edges[3].to_owned());
        new_edges.push(self.edges[2].chars().rev().collect());
        new_edges.push(self.edges[1].to_owned());
        self.edges = new_edges;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    let input_file = open_input(&filename);
    let tiles = read_input(input_file.unwrap()).unwrap();

    let corner_tiles: Vec<&Tile> = tiles.iter().filter(|t| t.is_corner_tile(&tiles)).collect();

    println!("corner_tiles {:?} ", corner_tiles);

    let answer: i64 = corner_tiles.iter().map(|t| t.id).product();

    println!("answer = {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let test_input = get_test_input().as_bytes();

        let tiles = read_input(test_input).unwrap();

        assert_eq!(tiles.len(), 9);
        assert_eq!(tiles[0].id, 2311);
        assert_eq!(tiles[0].edges[0], "..##.#..#.");
        assert_eq!(tiles[0].edges[1], "...#.##..#");
        assert_eq!(tiles[0].edges[2], "..###..###");
        assert_eq!(tiles[0].edges[3], ".#####..#.");
    }

    #[test]
    fn test_find_corner_tiles() {
        let test_input = get_test_input().as_bytes();

        let tiles = read_input(test_input).unwrap();

        let corner_tiles: Vec<&Tile> = tiles.iter().filter(|t| t.is_corner_tile(&tiles)).collect();

        assert_eq!(corner_tiles.len(), 4);

        let answer: i64 = corner_tiles.iter().map(|t| t.id).product();

        assert_eq!(answer, 20899048083289);
    }

    fn get_test_input() -> &'static str {
        "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."
    }
}
