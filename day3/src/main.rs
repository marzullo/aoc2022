use std::{io::Read, collections::HashSet, str::Split};

fn main() {
    let mut input_file = std::fs::OpenOptions::new().read(true).open("input.txt").expect("input fie not found");

    let mut input = String::new();

    input_file.read_to_string(&mut input);

    let mut lines = input.split("\r\n");

    part2(lines);
}

fn part1(lines: Split<&str>) {
    let mut sum = 0u32;

    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let len = line.len()/2;

        let ruck1 = &line[0..len];
        let ruck2 = &line[len..];

        let mut set1: HashSet<char> = HashSet::new();
        let mut set2: HashSet<char> = HashSet::new();

        ruck1.chars().for_each(|x| { set1.insert(x); });
        ruck2.chars().for_each(|x| { set2.insert(x); });

        let shared = set1.intersection(&set2);

        for char in shared {
            sum += get_char_value(*char);
        }
    }

    println!("{}", sum);
}

fn part2(mut lines: Split<&str>) {
    let mut sum = 0u32;

    while let Some(line1) = lines.next() {
        let line2 = lines.next().unwrap();
        let line3 = lines.next().unwrap();

        let mut set1: HashSet<char> = HashSet::new();
        let mut set2: HashSet<char> = HashSet::new();
        let mut set3: HashSet<char> = HashSet::new();

        line1.chars().for_each(|x| { set1.insert(x); });
        line2.chars().for_each(|x| { set2.insert(x); });
        line3.chars().for_each(|x| { set3.insert(x); });

        let intersect = set1.intersection(&set2).map(|x| x.clone()).collect::<HashSet<char>>();
        let intersect2 = intersect.intersection(&set3).collect::<Vec<&char>>();

        sum += get_char_value(*intersect2[0]);
    }

    println!("{}", sum);
}

fn get_char_value(character: char) -> u32 {
    println!("{}", character as u32);

    if character.is_ascii_lowercase() {
        return (character as u32) - 96;
    }

    return (character as u32) - 64 + 26;
}