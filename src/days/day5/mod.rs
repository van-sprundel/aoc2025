pub fn part1(input: &str) {
    println!("{}", solution(input, 1));
}

pub fn part2(input: &str) {
    println!("{}", solution(input, 0));
}

fn solution(input: &str, include_ids: usize) -> usize {
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

    // for branchless we need to sort (no-op if we use part1, doesn't hurt)
    ranges.sort_by_key(|(left, _)| *left);

    // merge overlapping ranges (idk if we could do this in sort but oh well)
    let merged = ranges.iter().fold(Vec::new(), |mut acc, &(left, right)| {
        let should_merge = acc
            .last()
            .map(|&(_, last_right): &(usize, usize)| left <= last_right + 1)
            .unwrap_or(false);

        let extend_last = should_merge as usize;
        let push_new = (1 - extend_last) as usize;

        // update last element if merging
        acc.iter_mut().last().map(|(_, last_right)| {
            *last_right = *last_right * (1 - extend_last) + extend_last * (*last_right).max(right);
        });

        // add new element if not merging
        acc.extend([(left, right)].iter().cycle().take(push_new));

        acc
    });

    // part 2 wants the sum of range sizes
    let part2_result: usize = merged.iter().map(|(left, right)| right - left + 1).sum();

    // part 1 wants to know which ids are valid in input
    let part1_result: usize = ids
        .lines()
        .map(|id| id.parse::<usize>().unwrap())
        .map(|id| {
            ranges
                .iter()
                .any(|(left, right)| id >= *left && id <= *right) as usize
        })
        .sum();

    //  multiply by include_ids to select result
    part1_result * include_ids + part2_result * (1 - include_ids)
}
