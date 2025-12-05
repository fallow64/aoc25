mod part1;
mod part2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Paper,
}

pub fn run() {
    let file = include_str!("day4.txt");

    let mut input = file
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '@' => Cell::Paper,
                    _ => panic!("invalid cell character"),
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();

    let result1 = part1::solve(&input);
    println!("Part 1: {result1}");

    let result2 = part2::solve(&mut input);
    println!("Part 2: {result2}");
}
