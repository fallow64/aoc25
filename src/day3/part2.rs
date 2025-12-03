// get maximum joltage (12 digits)

// problem: we are given a bank of batteries, and within each battery is a jolt.
// we need to create the maximum 12-digit joltage by selecting digits from the bank.
// and then sum those up
// greedy solution since we _always_ replace smaller digits with larger ones
pub fn solve(input: &Vec<Vec<usize>>) -> usize {
    const TARGET_DIGITS: usize = 12;

    input
        .iter()
        .map(|battery| {
            let digits = battery;
            let mut stack = Vec::new();
            let mut removals_left = digits.len() - TARGET_DIGITS;

            for &digit in digits {
                // remove smaller digits from stack if we can still afford to remove them
                while !stack.is_empty() && removals_left > 0 && *stack.last().unwrap() < digit {
                    stack.pop();
                    removals_left -= 1;
                }
                stack.push(digit);
            }

            // take only the first 12 digits from the stack
            stack
                .iter()
                .take(TARGET_DIGITS)
                .fold(0, |acc, &digit| acc * 10 + digit)
        })
        .sum()
}
