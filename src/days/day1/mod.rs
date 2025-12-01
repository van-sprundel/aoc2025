use std::{iter::Map, usize::MAX};

const DIAL_START: isize = 50;
const MAX_DIAL_LEN: isize = 100;

// s ∈ {+1, -1}
//
// L = -1 (char 76)
// R = +1 (char 82)
// s = 1 - 2 * ((c >> 2) & 1)
// s(L) = 1 - 2 * ((76 >> 2) & 1) = 1 - 2 * (19 & 1) = 1 - 2 * 1 = -1
// s(R) = 1 - 2 * ((76 >> 2) & 1) = 1 - 2 * (20 & 1) = 1 - 2 * 0 = 1
fn to_sign(c: u8) -> isize {
    (1 - 2 * ((c as isize >> 2) & 1))
}

pub fn part1(input: &str) {
    let mut count: usize = 0;
    let mut dial: isize = DIAL_START;

    input.lines().for_each(|line| {
        let num = line[1..].parse::<isize>().unwrap();
        let b = line.as_bytes()[0];

        dial += num * to_sign(b);
        dial = dial.rem_euclid(MAX_DIAL_LEN);
        count += (dial == 0) as usize;
    });

    println!("{count}")
}

pub fn part2(input: &str) {
    let mut count: usize = 0;
    let mut dial: isize = DIAL_START;

    input.lines().for_each(|line| {
        let num = line[1..].parse::<isize>().unwrap();
        let old_dial = dial;

        let b = line.as_bytes()[0];
        let dir_sign = to_sign(b);

        let raw = old_dial + dir_sign * num;
        let new_dial = raw.rem_euclid(MAX_DIAL_LEN);

        // if dir is left, then distance to 0 is old_dial
        // if dir is right, then distance to 0 is 100 - old_dial
        let distance_to_zero =
            (MAX_DIAL_LEN - old_dial) * (dir_sign + 1) / 2 + old_dial * (1 - dir_sign) / 2;

        // if num >= distance_to_zero, we hit zero at least once
        let hits_zero = (num >= distance_to_zero) as isize;

        // additional zeros after first hit
        let remaining = num - distance_to_zero;
        let additional_zeros = remaining.abs() / MAX_DIAL_LEN; // every remaining rounds is an extra 0

        // special case
        // if old_dial == 0, we don't count the "first" hit differently
        // this check is a bit annoying, so maybe TODO cleanup
        let at_zero = (old_dial == 0) as isize;
        let zeros_hit = hits_zero * (1 + additional_zeros) * (1 - at_zero) + at_zero * (num / 100);

        count += zeros_hit as usize;
        dial = new_dial;
    });

    println!("{count}")
}
