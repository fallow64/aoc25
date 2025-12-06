use super::Input;

pub fn solve(input: &Input) -> usize {
    input
        .columns
        .iter()
        .map(|(op, data)| match op {
            '+' => data.iter().sum::<usize>(),
            '*' => data.iter().product::<usize>(),
            _ => panic!("Unknown operator"),
        })
        .sum()
}
