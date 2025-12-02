use super::{FULL_ROTATION, STARTING_ROTATION};

// number of times where rotation is zero at any point during each change
pub fn solve(input: &[i32]) -> usize {
    let mut rotation = STARTING_ROTATION;
    let mut result_count = 0usize;

    for &delta in input {
        let step = if delta > 0 { 1 } else { -1 };
        let steps = delta.abs();

        // simple brute force simulation
        // is there a more clever way to do this? perchance.
        for _ in 0..steps {
            rotation += step;
            if rotation % FULL_ROTATION == 0 {
                result_count += 1;
            }
        }

        rotation = rotation % FULL_ROTATION;
    }

    result_count
}
