mod part1;
mod part2;

pub fn run(files: Vec<String>) {
    let file = files.first().expect("at least one input file");

    let input: Vec<Vec<usize>> = file
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit") as usize)
                .collect()
        })
        .collect();

    let result1 = part1::solve(&input);
    println!("Part 1: {result1}");

    let result2 = part2::solve(&input);
    println!("Part 2: {result2}");
}
