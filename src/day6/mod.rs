use regex::Regex;

mod part1;
mod part2;

#[derive(Debug)]
struct Input {
    pub columns: Vec<(char, Vec<usize>)>,
}

#[derive(Debug)]
struct OperatorInfo {
    pub index: usize,
    pub length: usize,
    pub operator: char,
}

pub fn run() {
    let file = include_str!("day6.txt");

    let input_simple = parse_input_simple(file);
    let result1 = part1::solve(&input_simple);
    println!("Part 1: {result1}");

    let input_by_column_index = parse_input_by_column_index(file);
    let result2 = part2::solve(&input_by_column_index);
    println!("Part 2: {result2}");
}

fn parse_input_simple(file: &str) -> Input {
    let lines: Vec<&str> = file.lines().collect();

    // split into data lines and operator line
    let data_lines: Vec<&str> = lines
        .iter()
        .take_while(|&&line| !line.contains('*') && !line.contains('+'))
        .copied()
        .collect();

    let operator_line = lines
        .iter()
        .find(|&&line| line.contains('*') || line.contains('+'))
        .expect("No operator line found");

    // parse numbers from data lines into a grid
    let grid: Vec<Vec<usize>> = data_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect()
        })
        .collect();

    // cetermine number of columns
    let num_cols = grid.iter().map(|row| row.len()).max().unwrap_or(0);

    // parse operators and assign column labels
    let operators: Vec<char> = operator_line
        .chars()
        .filter(|&c| c == '*' || c == '+')
        .collect();

    // transpose grid to get columns
    let mut columns = Vec::new();
    for col_idx in 0..num_cols {
        let column: Vec<usize> = grid
            .iter()
            .filter_map(|row| row.get(col_idx).copied())
            .collect();

        let operator = operators.get(col_idx).copied().unwrap_or('*');
        columns.push((operator, column));
    }

    Input { columns }
}

fn parse_input_by_column_index(file: &str) -> Input {
    // 1. find the operator line
    // 2. find the index and length for each operator
    // 3. for each operator, extract the numbers from that column based on the index and length
    // 4. assemble the numbers into the final input

    let lines: Vec<&str> = file.lines().collect();
    let operator_line = lines
        .iter()
        .find(|&&line| line.contains('*') || line.contains('+'))
        .expect("No operator line found");

    // parse operator positions using regex trickery
    let operators: Vec<OperatorInfo> = {
        let re = Regex::new(r"(\*|\+)\s*").unwrap();
        re.find_iter(operator_line)
            .enumerate()
            .map(|(_, mat)| OperatorInfo {
                index: mat.start(),
                length: mat.as_str().len()
                    - if mat.end() < operator_line.len() {
                        // subtract by one if is isn't the last operator (to account for trailing spaces between operators)
                        // little cursed lol
                        1
                    } else {
                        0
                    },
                operator: mat.as_str().chars().next().unwrap(),
            })
            .collect()
    };

    // now, for each column, extract the numbers from that column based on the index and length
    let mut columns = Vec::new();
    for op_info in &operators {
        let mut numbers_for_operator: Vec<usize> = Vec::new();

        for i in 0..op_info.length {
            // create the numbers column by column
            let mut digits_in_column = vec![];

            // extract the numbers from each line at the correct index
            for line in &lines {
                if line.len() >= op_info.index + i + 1 {
                    let num_str = &line[op_info.index + i..op_info.index + i + 1];
                    if let Ok(num) = num_str.trim().parse::<usize>() {
                        digits_in_column.push(num);
                    }
                }
            }

            // assemble the number for this column
            let number_in_column: usize = digits_in_column
                .iter()
                .fold(0, |acc, &digit| acc * 10 + digit);

            numbers_for_operator.push(number_in_column);
        }

        columns.push((op_info.operator, numbers_for_operator));
    }

    // dbg!("Operators: {:?}", operators);
    // dbg!("Columns: {:?}", &columns);

    Input { columns }
}
