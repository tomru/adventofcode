use std::fs;

fn main() {
    let input = fs::read_to_string("./data/input").expect("Unable to read file");
    let data = parse(&input);

    println!("Task 1: {:?}", count_inc(1, &data));
    println!("Task 2: {:?}", count_inc(3, &data));
}

fn count_inc(window_count: usize, data: &Vec<i32>) -> i32 {
    let mut last: i32 = i32::MAX;
    let mut counter = 0;
    for (pos, _) in data.iter().enumerate() {
        let e = get_last_n(pos, window_count, data)
            .into_iter()
            .reduce(|accum, item| accum + item)
            .unwrap_or(0);

        if last < e {
            counter += 1;
        }

        last = e;
    }
    return counter;
}

fn get_last_n(index: usize, n: usize, data: &Vec<i32>) -> Vec<i32> {
    let data_len = data.len();
    if index > data_len || index + n > data_len {
        return vec![];
    }
    return data.get(index..(n + index)).unwrap().to_vec();
}

fn parse(data: &String) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for line in data.lines() {
        result.push(line.parse::<i32>().unwrap());
    }

    return result;
}
