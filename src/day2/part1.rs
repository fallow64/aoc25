// the sum of invalid inputs
// where an invalid input is some sequence of digits repeated twice
pub fn solve(input: &[(usize, usize)]) -> usize {
    input
        .iter()
        // transform &[20-25, 30-32] to [20,21,22,23,24,25,30,31,32]
        .flat_map(|(start, end)| *start..=*end)
        // filter out valid numbers
        .filter(|&num| {
            let num_str = num.to_string();
            let midpoint = num_str.len() / 2;
            num_str.len() % 2 == 0 && &num_str[..midpoint] == &num_str[midpoint..]
        })
        // sum the invalid numbers
        .sum()
}
