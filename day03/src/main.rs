use std::fs;

const MAX_BITS: usize = 12;

fn main() {
    let input = fs::read_to_string("./data/input").expect("Unable to read file");
    let data = parse(&input);

    task1(&data);

    task2(&data);
}

fn task2(data: &Vec<Vec<bool>>) {
    println!("oxygen generator rating");
    let oxygen_generator_rating = get_rating(data, 0, true);
    println!("co2 scrubber");
    let co2_scrubber_rating = get_rating(data, 0, false);
    println!("live support rating {:?}", to_dec(&co2_scrubber_rating) * to_dec(&oxygen_generator_rating));
}

fn get_rating(data: &Vec<Vec<bool>>, index: usize, oxygen_flag: bool) -> Vec<bool> {
    let filter_value = most_common_at_index(data, index, oxygen_flag);
    println!("filter value {:?}", filter_value);
    let result: Vec<Vec<bool>> = data
        .iter()
        .filter(|v| v[index] == filter_value)
        .cloned()
        .collect();

    if index > data[0].len() {
        panic!("index out of bounds for first vector")
    }

    if result.len() == 1 {
        return result[0].clone();
    }

    return get_rating(&result, index + 1, oxygen_flag);
}

fn most_common_at_index(data: &Vec<Vec<bool>>, index: usize, oxygen_flag: bool) -> bool {
    let half = data.len() as f32 / 2 as f32;
    let true_count = data.into_iter().fold(0, |accum, item| {
        if *item.get(index).unwrap() {
            accum + 1
        } else {
            accum
        }
    });
    return if oxygen_flag { true_count as f32 >= half} else {half > true_count as f32};
}

fn to_dec(measurement: &Vec<bool>) -> u32 {
    return measurement
        .into_iter()
        .enumerate()
        .fold(0, |accum, (pos, item)| {
            let exp: u32 = (MAX_BITS - pos - 1) as u32;
            if *item {
                accum + (2 as u32).pow(exp)
            } else {
                accum
            }
        });
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

fn parse(data: &String) -> Vec<Vec<bool>> {
    let mut result: Vec<Vec<bool>> = vec![];
    for line in data.lines() {
        let thing: Vec<bool> = line
            .chars()
            .map(|v| if v == '1' { return true } else { return false })
            .collect();

        result.push(thing);
    }

    return result;
}

fn count(data: &Vec<Vec<bool>>, value: bool) -> Vec<u16> {
    let mut result = vec![0; data[0].len()];
    for item in data.iter() {
        for (pos, v) in item.iter().enumerate() {
            if *v == value {
                result[pos] += 1
            }
        }
    }

    return result;
}
