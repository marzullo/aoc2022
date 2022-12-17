use std::io::Read;

fn main() {
    let mut input_file = std::fs::OpenOptions::new().read(true).open("input.txt").expect("input file not found");
    
    let mut input = String::new();

    input_file.read_to_string(&mut input);

    let lines = input.split("\r\n");

    let mut current_dir_name = String::new();

    let mut dirs = Vec::<Dir>::new();
    let mut cur_dir: usize = 0;

    for line in lines {
        if line.contains("$ cd") {
            current_dir_name = line.strip_prefix("$ cd ").unwrap().to_string();

            if !dirs.iter().any(|x| x.name == current_dir_name) {
                dirs.push(Dir { name: String::from(&current_dir_name), file_sizes: Vec::new(), children: Vec::new() });

                cur_dir = dirs.len()-1;
            } else {
                cur_dir = dirs.iter().position(|x| x.name == current_dir_name).unwrap();
            }

            continue;
        }

        if line.contains("dir") {
            let new_dir = line.strip_prefix("dir ").unwrap().to_string();

            if !dirs.iter().any(|x| x.name == new_dir) {
                dirs.push(Dir { name: String::from(new_dir), file_sizes: Vec::new(), children: Vec::new() });
            }
            
            continue;
        }

        if line.contains("$ ls") {
            continue;
        }

        let mut split = line.split(" ");

        let file_size = split.next().unwrap();
        let file_name = split.next().unwrap();

        println!("dir: {} file: {:?} size: {:?}", current_dir_name.clone(), file_name, file_size);

        dirs[cur_dir].file_sizes.push(file_size.parse().unwrap());
    }

    let mut total = 0u32;

    for dir in dirs {
        let local_total = dir.file_sizes.iter().sum::<u32>();

        println!("{:?}", dir);

        if local_total <= 100000 {
            total += local_total;
        }
    }

    println!("total: {}", total);
}

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    file_sizes: Vec<u32>,
    children: Vec<Dir>
}
