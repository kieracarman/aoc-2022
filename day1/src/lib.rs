use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn a() -> String {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let (mut biggest_elf, mut current_elf) = (0, 0);

    for line in reader.lines() {
        let l = line.unwrap();

        if let "" = &*l {
            if current_elf > biggest_elf {
                biggest_elf = current_elf;
            }
            current_elf = 0;
        } else {
            current_elf += l.parse::<i32>().unwrap();
        }
    }

    biggest_elf.to_string()
}

pub fn b() -> String {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut current_elf = 0;

    let mut vec: Vec<i32> = vec![];

    for line in reader.lines() {
        let l = line.unwrap();

        if let "" = &*l {
            vec.push(current_elf);
            current_elf = 0;
        } else {
            current_elf += l.parse::<i32>().unwrap();
        }
    }

    vec.sort();

    let len = vec.len();

    let total = vec[len - 1] + vec[len - 2] + vec[len - 3];

    total.to_string()
}