mod part1;
mod part2;

struct Input {
    fresh_ranges: Vec<Range>,
    ids_to_check: Vec<usize>,
}

#[derive(PartialEq, Eq, Ord, Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.start != other.start {
            Some(self.start.cmp(&other.start))
        } else {
            Some(self.end.cmp(&other.end))
        }
    }
}

pub fn run() {
    let file = include_str!("day5.txt");

    let input = {
        let mut sections = file.split("\n\n");

        let fresh_ranges = sections
            .next()
            .expect("missing fresh ranges")
            .lines()
            .map(|line| {
                let (start, end) = line.split_once('-').expect("invalid range format");
                Range {
                    start: start.trim().parse().expect("start should be a number"),
                    end: end.trim().parse().expect("end should be a number"),
                }
            })
            .collect::<Vec<Range>>();

        let ids_to_check = sections
            .next()
            .expect("missing ids to check")
            .lines()
            .map(|line| line.trim().parse().expect("id should be a number"))
            .collect::<Vec<usize>>();

        Input {
            fresh_ranges,
            ids_to_check,
        }
    };

    let result1 = part1::solve(&input);
    println!("Part 1: {result1}");

    let result2 = part2::solve(&input);
    println!("Part 2: {result2}");
}
