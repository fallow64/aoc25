use super::{Input, Range};

pub fn solve(input: &Input) -> usize {
    let mut total_fresh = 0;

    let mut stack: Vec<Range> = {
        let mut ranges = input.fresh_ranges.clone();
        ranges.sort();
        ranges.reverse();
        ranges
    };

    while let Some(current_range) = stack.pop() {
        let mut merged_range = current_range.clone();

        while let Some(next_range) = stack.last() {
            if next_range.start <= merged_range.end + 1 {
                let next_range = stack.pop().unwrap();
                if next_range.end > merged_range.end {
                    merged_range.end = next_range.end;
                }
            } else {
                break;
            }
        }

        total_fresh += merged_range.end - merged_range.start + 1;
    }

    total_fresh
}
