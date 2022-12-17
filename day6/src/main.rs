use std::{io::Read, collections::{HashSet, VecDeque}};

fn main() {
    let mut input_file = std::fs::OpenOptions::new().read(true).open("input.txt").expect("no input found");

    let mut input = String::new();

    input_file.read_to_string(&mut input);

    let chars = input.chars();

    let mut set = VecDeque::<char>::new();

    let first_four = chars.clone().take(14);

    let first_four = first_four.map(|x| x.clone()).collect::<Vec<char>>();

    let mut idx = 0u32;
    let mut unique_count = 0u32;

    for char in first_four {
        if !set.contains(&char) {
            unique_count += 1;
        }

        idx += 1;
        set.push_back(char);
    }

    if unique_count == 14 {
        println!("{}", idx);
        return;
    }

    println!("unique count: {} set: {:?}", unique_count, set);

    for char in chars.skip(14) {
        set.pop_front();
        set.push_back(char);

        let check_set: HashSet<char> = HashSet::from_iter(set.clone().into_iter());

        if check_set.len() == 14 {
            println!("set: {:?} idx: {}", set, idx+1);

            return;
        }

        idx += 1;
    }
}
