use super::Cell;

pub fn solve(input: &Vec<Vec<Cell>>) -> usize {
    let start_pos = input
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(col_idx, cell)| match cell {
                    Cell::Start => Some((row_idx, col_idx)),
                    _ => None,
                })
        })
        .next()
        .expect("Input must have a starting position");

    let mut beams = vec![start_pos];
    let mut total_splits = 0;

    for _ in 0..input.len() {
        let mut new_beams = Vec::new();
        for &(row, col) in &beams {
            let cell = input[row][col];
            // dbg!((row, col, &cell));
            match cell {
                Cell::Empty | Cell::Start => {
                    if row + 1 < input.len() {
                        new_beams.push((row + 1, col));
                    }
                }
                Cell::Splitter => {
                    let mut is_new = false;
                    if !new_beams.contains(&(row + 1, col - 1)) {
                        is_new = true;
                        new_beams.push((row + 1, col - 1));
                    }

                    if !new_beams.contains(&(row + 1, col + 1)) {
                        is_new = true;
                        new_beams.push((row + 1, col + 1));
                    }

                    if is_new {
                        total_splits += 1;
                    }
                }
            }
        }

        // debug_print_grid(input, &new_beams);
        beams = new_beams;
    }

    total_splits
}
