use std::fs;

const MAX_BITS: usize = 12;

fn main() {
    let mut true_count: [ u16 ; MAX_BITS ] = [ 0 ; MAX_BITS ];
    let mut line_count: u16 = 0;
    let data = fs::read_to_string("./data/input").expect("Unable to read file");

    for line in data.lines() {
        let str_rev: String = line.chars().rev().collect();
        for (pos, chr) in str_rev.chars().enumerate() {
            if chr == '1' { 
                true_count[pos] += 1 
            }
        }
        line_count += 1;
    }

    let even_count = line_count / 2;
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for (pos, true_count) in true_count.iter_mut().enumerate() {
        if *true_count > even_count {
            gamma += (2 as u32).pow(pos as u32);
        } else {
            epsilon += (2 as u32).pow(pos as u32);
        }
    }
    println!("power consumption {}", epsilon * gamma)
}
