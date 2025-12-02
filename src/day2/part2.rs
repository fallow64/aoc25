// count invalid
// where invalid is if a substring is repeated _at all_
// i.e. 999 is 9 repeated 3 times, 824824824 is 824 repeated 3 times
pub fn solve(input: &[(usize, usize)]) -> usize {
    input
        .iter()
        // transform &[20-25, 30-32] to [20,21,22,23,24,25,30,31,32]
        .flat_map(|(start, end)| *start..=*end)
        // filter out valid numbers
        .filter(|&num| has_repeated_substring(&num.to_string()))
        // sum the invalid numbers
        .sum()
}

fn has_repeated_substring(s: &str) -> bool {
    let len = s.len();

    (1..=len / 2)
        // filter out lengths that don't divide evenly
        .filter(|&substr_len| len % substr_len == 0)
        .any(|substr_len| {
            // check if all chunks equal the first chunk
            let pattern = &s[..substr_len];
            s.as_bytes()
                .chunks(substr_len)
                .all(|chunk| chunk == pattern.as_bytes())
        })
}
