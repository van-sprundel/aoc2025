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

    for _ in 0..max_slots {
        if pos >= chars.len() {
            break;
        }

        // find the largest digit we can take from current position
        // also keep track of available slots
        let remaining_slots = max_slots - best.len();
        let remaining_chars = chars.len() - pos;

        if remaining_chars < remaining_slots {
            break;
        }

        // find the largest digit within valid range
        let max_look_ahead = remaining_chars - remaining_slots + 1;
        let mut best_digit = chars[pos];
        let mut best_idx = pos;

        for (i, curr) in chars.iter().enumerate().skip(pos).take(max_look_ahead) {
            if chars[i] > best_digit {
                best_digit = *curr;
                best_idx = i;
            }
        }

        best.push(best_digit);
        pos = best_idx + 1;
    }

    best.parse::<i64>().unwrap()
}
