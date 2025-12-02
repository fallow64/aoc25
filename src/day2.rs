pub fn run(files: Vec<String>) {
    let file = files.get(0).expect("at least one input file");

    // example input 10-20,30-40,50-60,... no newlines
    let input = file
        .trim()
        .split(',')
        .map(|pair| {
            let mut parts = pair.split('-');
            let start = parts
                .next()
                .expect("start of range")
                .trim()
                .parse::<usize>()
                .expect("start should be a number");
            let end = parts
                .next()
                .expect("end of range")
                .trim()
                .parse::<usize>()
                .expect("end should be a number");
            (start, end)
        })
        .collect::<Vec<(usize, usize)>>();

    let result1 = part1(&input);
    println!("Part 1: {result1}");

    let result2 = part2(&input);
    println!("Part 2: {result2}");
}

// the sum of invalid inputs
// where an invalid input is some sequence of digits repeated twice
fn part1(input: &[(usize, usize)]) -> usize {
    let mut invalid_sum = 0;

    for (range_start, range_end) in input {
        for num in *range_start..=*range_end {
            // detect if number is invalid
            let num_str = format!("{num}");
            // get middle point
            let midpoint = num_str.len() / 2;

            if &num_str[0..midpoint] == &num_str[midpoint..] {
                // is invalid, add it
                invalid_sum += num
            }
        }
    }

    invalid_sum
}

// count invalid
// where invalid is if a substring is repeated _at all_
// i.e. 999 = 9 repeated 3 times, 824824824 = 824 repeated 3 times
fn part2(input: &[(usize, usize)]) -> usize {
    let mut invalid_sum = 0;

    // check each range
    for (range_start, range_end) in input {
        // check each number in that range
        for num in *range_start..=*range_end {
            let num_str = num.to_string();
            let num_str_midpoint = num_str.len() / 2;

            // check each possible length
            for substr_length in 1..=num_str_midpoint {
                if num_str.len() % substr_length != 0 {
                    // not possible to be repeated since it does not divide nicely
                    continue;
                }

                let first_substr = &num_str[0..substr_length];
                // dbg!(&num_str, first_substr, num_str_midpoint);

                // create an iterator over chars, skipping the first substring (thats the thing we're checking against)
                let mut char_iter = num_str.chars().skip(substr_length).peekable();

                let mut is_not_repeated_substring = false;
                while char_iter.peek().is_some() {
                    let chunk: String = char_iter.by_ref().take(substr_length).collect();
                    if chunk != first_substr {
                        is_not_repeated_substring = true;
                        break;
                    }
                }

                if !is_not_repeated_substring {
                    println!("INVALID: {num}");
                    // is invalid, add it to the sum
                    invalid_sum += num;
                    // don't count this number multiple times; move onto the next number in the range
                    break;
                }
            }
        }
    }

    invalid_sum
}
