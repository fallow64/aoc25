use super::{FULL_ROTATION, STARTING_ROTATION};

// number of times where rotation is zero after each change
pub fn solve(input: &[i32]) -> usize {
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
