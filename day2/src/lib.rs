use std::collections::HashMap;

pub fn a(input: &str) -> i32 {
    let legend = HashMap::from([
        ("BZ".to_string(), [2, 9]),
        ("AY".to_string(), [1, 8]),
        ("CX".to_string(), [3, 7]),
        ("CZ".to_string(), [6, 6]),
        ("BY".to_string(), [5, 5]),
        ("AX".to_string(), [4, 4]),
        ("AZ".to_string(), [7, 3]),
        ("CY".to_string(), [9, 2]),
        ("BX".to_string(), [8, 1]),
    ]);

    input
        .lines()
        .map(|line| {
            legend[&line
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()][1]
        })
        .sum()

    // let mut score = 0;

    // for line in input.lines() {
    //     let key: String = line.chars().filter(|c| !c.is_whitespace()).collect();
    //     score += legend[&key][1];
    // }

    // score
}

pub fn b(input: &str) -> i32 {
    let legend: HashMap<String, [i32; 2]> = HashMap::from([
        ("BZ".to_string(), [2, 9]),
        ("AZ".to_string(), [1, 8]),
        ("CZ".to_string(), [3, 7]),
        ("CY".to_string(), [6, 6]),
        ("BY".to_string(), [5, 5]),
        ("AY".to_string(), [4, 4]),
        ("AX".to_string(), [7, 3]),
        ("CX".to_string(), [9, 2]),
        ("BX".to_string(), [8, 1]),
    ]);

    input
        .lines()
        .map(|line| {
            legend[&line
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()][1]
        })
        .sum()

    // let mut score = 0;

    // for line in input.lines() {
    //     let key: String = line.chars().filter(|c| !c.is_whitespace()).collect();
    //     score += legend[&key][1];
    // }

    // score
}

// let score_legend = {
//     "A": 1, // rock
//     "B": 2, // paper
//     "C": 3, // scissors
//     "X": 1, // rock
//     "Y": 2, // paper
//     "Z": 3  // scissors
// };

// part a
// BZ  win     2, 9
// AY  win     1, 8
// CX  win     3, 7
// CZ  draw    6, 6
// BY  draw    5, 5
// AX  draw    4, 4
// AZ  lose    7, 3
// CY  lose    9, 2
// BX  lose    8, 1

// part b
// BZ  win     2, 9
// AZ  win     1, 8
// CZ  win     3, 7
// CY  draw    6, 6
// BY  draw    5, 5
// AY  draw    4, 4
// AX  lose    7, 3
// CX  lose    9, 2
// BX  lose    8, 1
