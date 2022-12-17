use std::{io::Read, collections::HashMap};

fn main() {
    let mut input_file = std::fs::OpenOptions::new().read(true).open("input.txt").expect("input.txt not found");

    let mut input = String::new();

    input_file.read_to_string(&mut input).unwrap();

    let score = parse_input_2(input);

    println!("{}", score);
}

fn parse_input(input: String) -> u64 {
    let lines = input.split("\n");

    let mut p2_score = 0u64;

    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let mut moves = line.split(" ");
        
        let p1_move = moves.next().unwrap();
        let p2_move = match moves.next().unwrap() {
            "X" => "A",
            "Y" => "B",
            "Z" => "C",
            _ => ""
        };

        p2_score += match p2_move {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => 0
        };

        // draw condition
        if p1_move == p2_move {
            p2_score += 3;

            continue;
        }

        p2_score += match (p2_move, p1_move) {
            ("A", "C") => 6,
            ("B", "A") => 6,
            ("C", "B") => 6,
            (_, _) => 0
        };
    }

    p2_score
}

fn parse_input_2(input: String) -> u64 {
    let lines = input.split("\n");

    let mut p2_score = 0u64;

    for line in lines {
        if line.len() == 0 {
            continue;
        }

        let mut moves = line.split(" ");
        
        let p1_move = moves.next().unwrap();
        p2_score += match (moves.next().unwrap(), p1_move) {
            ("Y", "A") => 3 + 1,//"A",
            ("Y", "B") => 3 + 2,//"B",
            ("Y", "C") => 3 + 3,//"C",
            ("Z", "A") => 6 + 2,//"B",
            ("Z", "B") => 6 + 3,//"C",
            ("Z", "C") => 6 + 1,//"A",
            ("X", "A") => 3,//"C",
            ("X", "B") => 1,//"A",
            ("X", "C") => 2,//"B",
            (_, _) => 0
        };
    }

    p2_score
}
