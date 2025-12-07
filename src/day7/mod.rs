mod part1;
mod part2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Start,
    Splitter,
}

pub fn run() {
    let file = include_str!("day7.txt");

    let input = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '^' => Cell::Splitter,
                    'S' => Cell::Start,
                    _ => panic!("Unexpected character in input"),
                })
                .collect::<Vec<Cell>>()
        })
        .collect::<Vec<Vec<Cell>>>();

    let result1 = part1::solve(&input);
    println!("Part 1: {result1}");

    let result2 = part2::solve(&input);
    println!("Part 2: {result2}");
}
