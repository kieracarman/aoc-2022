use std::fs::File;
use std::io::{BufRead, BufReader, Result};

// part 1

// fn main() -> Result<()> {
//     let file = File::open("input.txt")?;
//     let reader = BufReader::new(file);

//     let (mut biggest_elf, mut current_elf) = (0, 0);

//     for line in reader.lines() {
//         let l = line.unwrap();

//         if let "" = &*l {
//             if current_elf > biggest_elf {
//                 biggest_elf = current_elf;
//             }
//             current_elf = 0;
//         } else {
//             current_elf += l.parse::<i32>().unwrap();
//         }
//     }

//     Ok(println!("{}", biggest_elf))
// }

// part 2

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
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

    Ok(println!("{}", total))
}