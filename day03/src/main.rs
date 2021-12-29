use std::fs;

const MAX_BITS: usize = 12;

fn main() {
    let input = fs::read_to_string("./data/input").expect("Unable to read file");
    let data = parse(&input);

    task1(&data);

    task2(&data); 
}

fn task2(data: &Vec<Vec<bool>>) {
    let a = filter_by_first_bit_n(data, true, 0);

    println!("{:?}", a);
}

fn task1(data: &Vec<Vec<bool>>) {
    let trues = count(&data, true);

    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for (pos, true_count) in trues.iter().enumerate() {
        let exp: u32 = (MAX_BITS - pos - 1) as u32;
        if *true_count > data.len() as u16 / 2 {
            gamma += (2 as u32).pow(exp);
        } else {
            epsilon += (2 as u32).pow(exp);
        }
    }
    println!("power consumption {}", epsilon * gamma);
}

fn filter_by_first_bit_n(data: &Vec<Vec<bool>>, value: bool, n: usize) -> Vec<bool> {
    let result: Vec<Vec<bool>> = data.iter().filter(|v| v[n] == value).cloned().collect();

    if result.len() == 1 {
        return result[0].clone();
    }

    return filter_by_first_bit_n(&result, value, n+1);
}

fn parse(data: &String) -> Vec<Vec<bool>> {
    let mut result: Vec<Vec<bool>> = vec![];
    for line in data.lines() {
        let thing: Vec<bool> = line.chars()
            .map(|v| if v == '1' { return true } else { return  false })
            .collect();

        result.push(thing);
    }

    return result;
}

fn count(data: &Vec<Vec<bool>>, value: bool) -> Vec<u16> {
    let mut result = vec![0 ; data[0].len()];
    for item in data.iter() {
        for (pos, v) in item.iter().enumerate() {
            if *v == value { result[pos] += 1 }
        }
    }

    return result;
}

