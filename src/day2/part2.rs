// count invalid
// where invalid is if a substring is repeated _at all_
// i.e. 999 = 9 repeated 3 times, 824824824 = 824 repeated 3 times
pub fn solve(input: &[(usize, usize)]) -> usize {
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
            s.as_bytes().chunks(substr_len).all(|chunk| chunk == pattern.as_bytes())
        })
}
