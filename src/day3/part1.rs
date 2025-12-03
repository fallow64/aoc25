// get maximum joltage (two digits)
pub fn solve(input: &Vec<Vec<usize>>) -> usize {
    let mut sum = 0usize;
    for bank in input {
        // now find maximum combination of 2 digits
        // this is a bruteforce solution
        let mut max_combo = 0usize;
        for i in 0..bank.len() {
            for j in i + 1..bank.len() {
                let combo = bank[i] * 10 + bank[j];
                if combo > max_combo {
                    max_combo = combo;
                }
            }
        }

        sum += max_combo as usize;
    }

    sum
}
