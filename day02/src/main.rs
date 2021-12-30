use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut depth: i32 = 0;
    let mut pos_h: i32 = 0;
    let mut aim: i32 = 0;
    if let Ok(lines) = read_lines("./data/input") {
        for line in lines {
            if let Ok(str) = line {
                let words: Vec<&str> = str.split_whitespace().collect();
                let direction = words[0];
                let number = words[1].parse::<i32>().unwrap();

                if direction == "down" {
                    aim += number;
                } else if direction == "up" {
                    aim -= number;
                } else if direction == "forward" {
                    depth += number * aim;
                    pos_h += number;
                } else {
                    eprintln!("UNKNOWN!!!!!")
                }
            }
        }
    }
    let result = depth * pos_h;
    println!("{}", result);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
