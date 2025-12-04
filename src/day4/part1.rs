use super::Cell;

pub fn solve(input: &Vec<Vec<Cell>>) -> usize {
    let mut count = 0;

    for row in 0..input.len() {
        for col in 0..input[row].len() {
            if input[row][col] != Cell::Paper {
                continue;
            }

            let mut neighbors = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }
                    let nr = row as isize + dr;
                    let nc = col as isize + dc;

                    if nr >= 0
                        && nr < input.len() as isize
                        && nc >= 0
                        && nc < input[row].len() as isize
                        && input[nr as usize][nc as usize] == Cell::Paper
                    {
                        neighbors += 1;
                    }
                }
            }

            if neighbors < 4 {
                println!("Cell at ({},{}) has only {} neighbors", row, col, neighbors);
                count += 1;
            }
        }
    }

    count
}
