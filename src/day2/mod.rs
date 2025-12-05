mod part1;
mod part2;

pub fn run() {
    let file = include_str!("day2.txt");

    let input: Vec<(usize, usize)> = file
        .trim()
        .split(',')
        .map(|pair| {
            let (start, end) = pair.split_once('-').expect("invalid range format");
            (
                start.trim().parse().expect("start should be a number"),
                end.trim().parse().expect("end should be a number"),
            )
        })
        .collect();

    let result1 = part1::solve(&input);
    println!("Part 1: {result1}");

    let result2 = part2::solve(&input);
    println!("Part 2: {result2}");
}
