use std::collections::HashSet;

pub fn part1(input: &str) {
    println!("{}", solution(input, true));
}

pub fn part2(input: &str) {
    println!("{}", solution(input, false));
}

fn solution(input: &str, include_ids: bool) -> usize {
    let [ranges, ids] = input.split("\n\n").collect::<Vec<_>>()[..] else {
        panic!("Invalid input")
    };

    let mut ranges: Vec<(usize, usize)> = ranges
        .lines()
        .map(|range| {
            let [left, right] = range
                .split('-')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>()[..]
            else {
                panic!("Invalid range")
            };
            (left, right)
        })
        .collect();

    if !include_ids {
        // count total IDs in all ranges (need to merge overlapping ranges)
        ranges.sort_by_key(|(left, _)| *left);

        let mut merged: Vec<(usize, usize)> = Vec::new();
        for (left, right) in ranges {
            if let Some(last) = merged.last_mut() {
                if left <= last.1 + 1 {
                    last.1 = last.1.max(right);
                } else {
                    merged.push((left, right));
                }
            } else {
                merged.push((left, right));
            }
        }

        return merged.iter().map(|(left, right)| right - left + 1).sum();
    }

    // Part 1: Count IDs from list that fall in ranges
    let mut count = 0;
    for id in ids.lines().map(|id| id.parse::<usize>().unwrap()) {
        count += (ranges
            .iter()
            .any(|(left, right)| id >= *left && id <= *right)) as usize;
    }

    count
}
