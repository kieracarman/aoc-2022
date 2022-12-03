pub fn a(input: &str) -> i32 {
    let legend = std::collections::HashMap::from([
        ("B Z", 9),
        ("A Y", 8),
        ("C X", 7),
        ("C Z", 6),
        ("B Y", 5),
        ("A X", 4),
        ("A Z", 3),
        ("C Y", 2),
        ("B X", 1),
    ]);

    input.lines().map(|line| legend[line]).sum()
}

pub fn b(input: &str) -> i32 {
    let legend = std::collections::HashMap::from([
        ("B Z", 9),
        ("A Z", 8),
        ("C Z", 7),
        ("C Y", 6),
        ("B Y", 5),
        ("A Y", 4),
        ("A X", 3),
        ("C X", 2),
        ("B X", 1),
    ]);

    input.lines().map(|line| legend[line]).sum()
}
