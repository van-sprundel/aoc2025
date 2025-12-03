pub fn part1(input: &str) {
    let result = input.lines().map(|line| solution(line, 2)).sum::<i64>();

    println!("{result}");
}
pub fn part2(input: &str) {
    let result = input.lines().map(|line| solution(line, 12)).sum::<i64>();

    println!("{result}");
}

fn solution(line: &str, max_slots: usize) -> i64 {
    let mut best = String::new();
    let mut pos = 0;
    let chars: Vec<char> = line.chars().collect();

    for _ in 0..max_slots.min(chars.len()) {
        let remaining_slots = max_slots - best.len();
        let remaining_chars = chars.len() - pos;

        //  check if we should continue
        let should_continue = (remaining_chars >= remaining_slots) as usize;

        // calculate max_look_ahead (will be 0 if we shouldn't continue)
        let max_look_ahead = (remaining_chars - remaining_slots + 1) * should_continue;

        // find the largest digit within valid range
        let (best_digit, best_idx) = chars
            .iter()
            .enumerate()
            .skip(pos)
            .take(max_look_ahead)
            .fold(
                (chars.get(pos).copied().unwrap_or('0'), pos),
                |(best_digit, best_idx), (i, &curr)| {
                    // select the better digit/index
                    let is_better = (curr > best_digit) as usize;
                    let new_digit = [best_digit, curr][is_better];
                    let new_idx = [best_idx, i][is_better];
                    (new_digit, new_idx)
                },
            );

        // only push if we actually found something (max_look_ahead > 0)
        best.extend(std::iter::repeat_n(best_digit, should_continue));
        pos = best_idx + should_continue;
    }

    best.parse::<i64>().unwrap()
}
