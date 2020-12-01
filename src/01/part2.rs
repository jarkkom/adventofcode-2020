use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::env;

fn read_input(filename: &str) -> Vec<i64> {
    let path = Path::new(filename);

    println!("{:?}", path);

    let file = File::open(path);

    println!("{:?}", file);

    let reader = BufReader::new(file.unwrap());

    let mut output = Vec::new();

    for line_iter in reader.lines() {
        let intval: i64 = line_iter.unwrap().parse().unwrap();
        output.push(intval);
    }

    return output;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).unwrap();

    println!("{}", filename);

    let inputs = read_input(&filename);

    for i in 0..inputs.len() {
        for j in i + 1..inputs.len() {
            for k in i + 1..inputs.len() {
                let a = inputs.get(i).unwrap();
                let b = inputs.get(j).unwrap();
                let c = inputs.get(k).unwrap();
                if a + b + c == 2020 {
                    println!("{} + {} + {} = {} -> {}", a, b, c, a + b + c, a * b * c);
                }
            }
        }
    }

    println!("{:?}", inputs);
}