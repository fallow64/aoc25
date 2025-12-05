use super::Input;

pub fn solve(input: &Input) -> usize {
    let mut fresh_count = 0;

    for id in &input.ids_to_check {
        for range in &input.fresh_ranges {
            if id >= &range.start && id <= &range.end {
                fresh_count += 1;
                break;
            }
        }
    }

    fresh_count
}
