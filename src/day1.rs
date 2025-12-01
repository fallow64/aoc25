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

    let result1 = part1(&input);
    println!("Part 1: {result1}");

    let result2 = part2(&input);
    println!("Part 2: {result2}");
}

// number of times where rotation is zero after each change
fn part1(input: &[i32]) -> usize {
    let mut rotation = STARTING_ROTATION;
    let mut result_count = 0usize;

    for delta in input {
        rotation = (rotation + delta) % FULL_ROTATION;
        if rotation == 0 {
            result_count += 1;
        }
    }

    result_count
}

// number of times where rotation is zero at any point during each change
fn part2(input: &[i32]) -> usize {
    let mut rotation = STARTING_ROTATION;
    let mut result_count = 0usize;

    for &delta in input {
        let step = if delta > 0 { 1 } else { -1 };
        let steps = delta.abs();

        // simple brute force simulation
        // is there a more clever way to do this? perchance.
        for _ in 0..steps {
            rotation += step;
            if rotation % FULL_ROTATION == 0 {
                result_count += 1;
            }
        }

        rotation = rotation % FULL_ROTATION;
    }

    result_count
}
