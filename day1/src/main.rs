use std::io::Read;

fn main() {
    let mut input_file = std::fs::OpenOptions::new()
        .read(true)
        .open("input.txt")
        .expect("input.txt not found");

    let mut input = String::new();

    input_file.read_to_string(&mut input).unwrap();

    let result = parse_input_part_one(input.clone());

    println!("{}", result);

    let result = parse_input_part_two(input);

    println!("{}", result.iter().sum::<u64>());
}

fn parse_input_part_one(input: String) -> u64 {
    let lines = input.split("\n");

    let mut max_sum = 0u64;
    let mut current_sum = 0u64;

    for line in lines {
        if line.len() == 0 {
            if current_sum > max_sum {
                max_sum = current_sum;
            }

            current_sum = 0;
        } else {
            let calories: u64 = line.parse().unwrap();
            current_sum += calories;
        }
    }

    max_sum
}

fn determine_maxes(calories: u64, mut maxes: [u64; 3]) -> [u64; 3] {
    let min = maxes.iter_mut().min().unwrap();

    if calories < *min {
        return maxes;
    }

    *min = calories;

    maxes
}

fn parse_input_part_two(input: String) -> [u64; 3] {
    let lines = input.split("\n");

    let mut max_sums = [0u64, 0u64, 0u64];
    let mut current_sum = 0u64;

    for line in lines {
        if line.len() == 0 {
            max_sums = determine_maxes(current_sum, max_sums);

            current_sum = 0;
        } else {
            let calories: u64 = line.parse().unwrap();
            current_sum += calories;
        }
    }

    max_sums
}
