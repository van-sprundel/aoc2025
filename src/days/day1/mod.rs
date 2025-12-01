pub fn part1(input: &str) {
    let mut count: usize = 0;
    let mut dial: isize = 50;

    input.lines().for_each(|line| {
        if line.len() < 2 {
            return;
        }
        let num = line[1..].parse::<isize>().unwrap();
        if line.starts_with("L") {
            dial -= num;
        } else {
            dial += num;
        }

        dial = dial.rem_euclid(100);
        if dial == 0 {
            count += 1
        }
    });

    println!("{count}")
}

pub fn part2(input: &str) {
    let mut count: usize = 0;
    let mut dial: isize = 50;

    input.lines().for_each(|line| {
        if line.len() < 2 {
            return;
        }

        let num = line[1..].parse::<isize>().unwrap();
        let old_dial = dial;

        let new_dial = if line.starts_with("L") {
            ((old_dial - num) % 100 + 100) % 100
        } else {
            (old_dial + num) % 100
        };

        // we need to count how many times we land on 0 during this rotation
        // including final position
        let zeros_hit: isize;

        if line.starts_with("L") {
            // moving left: old_dial → old_dial-1 → ... → new_dial
            // from position old_dial, going left num steps

            // we hit 0 when i = old_dial, old_dial + 100, old_dial + 200, ...
            // for i in range [1, num]
            if old_dial == 0 {
                zeros_hit = num / 100;
            } else {
                // first time we hit 0 is at step old_dial
                // then every 100 steps after that
                if num >= old_dial {
                    zeros_hit = 1 + ((num - old_dial) / 100);
                } else {
                    zeros_hit = 0;
                }
            }
        } else {
            // moving right: old_dial → old_dial+1 → ... → new_dial
            // from position old_dial, going right num steps

            // we hit 0 when (old_dial + i) mod 100 == 0 for i in 1..=num
            // this happens at i = 100-old_dial, 200-old_dial, etc.
            if old_dial == 0 {
                zeros_hit = num / 100;
            } else if num >= (100 - old_dial) {
                zeros_hit = 1 + ((num - (100 - old_dial)) / 100);
            } else {
                zeros_hit = 0;
            }
        }

        count += zeros_hit as usize;
        dial = new_dial;
    });

    println!("{count}")
}
