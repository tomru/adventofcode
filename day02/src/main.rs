use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut depth = 0;
    let mut pos_h = 0;
    if let Ok(lines) = read_lines("./data/input") {
        for line in lines {
            if let Ok(str) = line {
                let words: Vec<&str> = str.split_whitespace().collect();
                let direction = words[0];
                let number = words[1].parse::<i32>().unwrap();

                match direction {
                    "down" => depth = depth + number,
                    "up" => depth = depth - number,
                    "forward" => pos_h = pos_h + number,
                    _ => eprintln!("UNKNOWN!!!!!"),

                }
            }
        }
    }
    println!("{}", depth * pos_h);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
