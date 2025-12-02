mod part1;
mod part2;

const STARTING_ROTATION: i32 = 50;
const FULL_ROTATION: i32 = 100;

pub fn run(files: Vec<String>) {
    let file = files.get(0).expect("at least one input file");
    let input = file
        .lines()
        .map(|line| {
            let direction = match &line[0..1] {
                "L" => -1,
                "R" => 1,
                _ => panic!("invalid direction"),
            };

            let distance = line[1..]
                .trim()
                .parse::<i32>()
                .expect("distance should be a number");

            direction * distance
        })
        .collect::<Vec<i32>>();

    let result1 = part1::solve(&input);
    println!("Part 1: {result1}");

    let result2 = part2::solve(&input);
    println!("Part 2: {result2}");
}
