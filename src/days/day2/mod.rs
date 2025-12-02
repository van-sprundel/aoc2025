pub fn part1(input: &str) {
    solution(input, Part::PART1)
}
pub fn part2(input: &str) {
    solution(input, Part::PART2)
}

fn solution(input: &str, part: Part) {
    let mut invalid_id_acc = 0;
    input.split(',').for_each(|range| {
        let divider_index = range.find('-');
        if let Some(index) = divider_index {
            let left = range[..index].trim().parse::<usize>().unwrap();
            let right = range[index + 1..].trim().parse::<usize>().unwrap();

            let mut index = left;
            loop {
                if index > right {
                    break;
                }

                if !is_valid_id(&index.to_string(), part.clone()) {
                    invalid_id_acc += index;
                }

                index += 1;
            }
        }
    });

    println!("{invalid_id_acc}")
}

#[derive(PartialEq, Clone)]
enum Part {
    PART1,
    PART2,
}

fn is_valid_id(n: &str, part: Part) -> bool {
    let bytes = n.as_bytes();
    let len = bytes.len();

    // we now need to find all denominators i guess because i can match three four and five times
    // so len/2 can match, but also len/3 len/4 .. len/n (1)
    let (start_d, end_d) = match part {
        /// s == s[0:2] repeated n/2 times
        Part::PART1 => {
            if len % 2 == 1 {
                return true;
            }
            let d = len / 2;
            (d, d + 1)
        }
        /// s == s[0:d] repeated n/d times
        Part::PART2 => (1, len),
    };
    for d in start_d..end_d {
        // this can NEVER match since we can't get d chunks of the string
        // e.g. 1231234 we cannot get a left,right slice because it's uneven
        // this is fine to skip
        if !len.is_multiple_of(d) {
            continue;
        }

        let chunk = &bytes[..d];
        let mut ok = true;

        let mut i = d;
        while i < len {
            if &bytes[i..i + d] != chunk {
                ok = false;
                break;
            }
            i += d;
        }

        if ok {
            return false;
        }
    }

    true
}
