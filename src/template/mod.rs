mod part1;
mod part2;

pub fn run() {
    let file = "";

    let result1 = part1::solve(&file);
    println!("Part 1: {result1}");

    let result2 = part2::solve(&file);
    println!("Part 2: {result2}");
}
