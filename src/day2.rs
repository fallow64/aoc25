pub fn run(files: Vec<String>) {
    let file = files.first().expect("at least one input file");

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

    let result1 = part1(&input);
    println!("Part 1: {result1}");

    let result2 = part2(&input);
    println!("Part 2: {result2}");
}

// the sum of invalid inputs
// where an invalid input is some sequence of digits repeated twice
fn part1(input: &[(usize, usize)]) -> usize {
    input
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .filter(|&num| {
            let num_str = num.to_string();
            let midpoint = num_str.len() / 2;
            num_str.len() % 2 == 0 && &num_str[..midpoint] == &num_str[midpoint..]
        })
        .sum()
}

// count invalid
// where invalid is if a substring is repeated _at all_
// i.e. 999 = 9 repeated 3 times, 824824824 = 824 repeated 3 times
fn part2(input: &[(usize, usize)]) -> usize {
    input
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .filter(|&num| has_repeated_substring(&num.to_string()))
        .sum()
}

fn has_repeated_substring(s: &str) -> bool {
    let len = s.len();

    (1..=len / 2)
        .filter(|&substr_len| len % substr_len == 0)
        .any(|substr_len| {
            let pattern = &s[..substr_len];
            s.as_bytes()
                .chunks(substr_len)
                .all(|chunk| chunk == pattern.as_bytes())
        })
}
