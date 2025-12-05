pub fn part1(s: &str) {
    println!("{}", solve(s, false));
}

pub fn part2(s: &str) {
    println!("{}", solve(s, true));
}

fn solve(s: &str, remove: bool) -> usize {
    let original: Vec<Vec<u8>> = s.lines().map(|l| l.as_bytes().to_vec()).collect();
    let h = original.len();
    let w = original[0].len();

    // pad grid with 1-cell border
    let mut g = vec![vec![b'.'; w + 2]; h + 2];
    for y in 0..h {
        for x in 0..w {
            g[y + 1][x + 1] = original[y][x];
        }
    }

    let dirs: [(isize, isize); 8] = [
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ];

    let mut total = 0;
    let mut should_continue = 1;

    while should_continue != 0 {
        let mut found_this_round = 0;
        let mut to_remove = vec![vec![0u8; w + 2]; h + 2];

        for y in 1..=h {
            for x in 1..=w {
                let is_at = (g[y][x] == b'@') as usize;

                let mut count = 0;
                for &(dy, dx) in &dirs {
                    let ny = (y as isize + dy) as usize;
                    let nx = (x as isize + dx) as usize;
                    count += (g[ny][nx] == b'@') as usize;
                }

                let should_remove = is_at * ((count < 4) as usize);
                to_remove[y][x] = should_remove as u8;
                found_this_round += should_remove;
            }
        }

        total += found_this_round;

        // replace with '.' if marked, keep current otherwise
        // we need to multiply by 0 to skip when should_continue will be 0
        let do_removal = (remove as usize) * ((found_this_round > 0) as usize);
        for y in 1..=h {
            for x in 1..=w {
                let mask = to_remove[y][x] * (do_removal as u8);
                g[y][x] = g[y][x] * (1 - mask) + b'.' * mask;
            }
        }

        // if not removing, stop after first iteration
        // if removing, stop when nothing found
        should_continue = (remove as usize) * ((found_this_round > 0) as usize);
    }

    total
}
