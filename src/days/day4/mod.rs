pub fn part1(s: &str) {
    let g: Vec<Vec<u8>> = s.lines().map(|l| l.as_bytes().to_vec()).collect();
    let h = g.len();
    let w = g[0].len();

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

    for y in 0..h {
        for x in 0..w {
            if g[y][x] != b'@' {
                continue;
            }

            let mut count = 0;
            for (dy, dx) in dirs {
                let ny = y as isize + dy;
                let nx = x as isize + dx;
                if ny < 0 || nx < 0 || ny >= h as isize || nx >= w as isize {
                    continue;
                }
                if g[ny as usize][nx as usize] == b'@' {
                    count += 1;
                }
            }

            if count < 4 {
                result += 1;
            }
        }
    }

    println!("{result}");
}

pub fn part2(input: &str) {
    todo!("Day 4 part 2 not implemented yet");
}
