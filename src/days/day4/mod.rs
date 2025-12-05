pub fn part1(s: &str) {
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

    let mut result = 0;

    for y in 1..=h {
        for x in 1..=w {
            // 1 if '@', 0 otherwise
            let is_at = (g[y][x] == b'@') as usize;

            let mut count = 0;
            for &(dy, dx) in &dirs {
                let ny = (y as isize + dy) as usize;
                let nx = (x as isize + dx) as usize;

                // no bounds check needed due to padding B)
                count += (g[ny][nx] == b'@') as usize;
            }

            // multiply conditions together
            result += is_at * ((count < 4) as usize);
        }
    }

    println!("{result}");
}

pub fn part2(input: &str) {
    todo!("Day 4 part 2 not implemented yet");
}
