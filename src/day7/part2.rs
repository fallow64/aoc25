use super::Cell;

pub fn solve(input: &Vec<Vec<Cell>>) -> usize {
    let n = input.len();
    let m = input[0].len();

    // number of paths that reach cell (i, j)
    let mut dp = vec![vec![0usize; m]; n];

    // initialize first row with starting position
    for j in 0..m {
        if input[0][j] == Cell::Start {
            dp[0][j] = 1;
        }
    }

    for i in 1..n {
        for j in 0..m {
            if input[i][j] != Cell::Splitter {
                // path comes straight down
                dp[i][j] += dp[i - 1][j];
            } else {
                // split left and right
                if j > 0 {
                    dp[i][j - 1] += dp[i - 1][j];
                }
                if j + 1 < m {
                    dp[i][j + 1] += dp[i - 1][j];
                }
            }
        }
    }

    // sum last row
    dp[n - 1].iter().sum()
}
