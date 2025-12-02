pub fn part1(input: &str) {
    let mut invalid_id_acc = 0;
    input.split(',').for_each(|range| {
        let divider_index = range.find('-');
        if let Some(index) = divider_index {
            let left = range[..index].trim().parse::<usize>().unwrap();
            let right = range[index + 1..].trim().parse::<usize>().unwrap();

            let mut index = left.clone();
            loop {
                if index > right {
                    break;
                }

                if !is_valid_id(&*index.to_string()) {
                    invalid_id_acc += index;
                }

                index += 1;
            }
        }
    });

    println!("{invalid_id_acc}")
}

pub fn part2(input: &str) {
    let mut invalid_id_acc = 0;

    input.split(',').for_each(|range| {
        let divider_index = range.find('-');
        if let Some(index) = divider_index {
            let left = range[..index].trim().parse::<usize>().unwrap();
            let right = range[index + 1..].trim().parse::<usize>().unwrap();

            let mut index = left.clone();
            loop {
                if index > right {
                    break;
                }

                if !is_valid_id_part2(&*index.to_string()) {
                    invalid_id_acc += index;
                }

                index += 1;
            }
        }
    });

    println!("{invalid_id_acc}")
}

fn is_valid_id(n: &str) -> bool {
    let chars = n.chars().collect::<Vec<_>>();
    let len = chars.len();

    // it can't be the same code twice if the char lengths are uneven
    // since if we grab left and right they will never match
    if len % 2 == 1 {
        return true;
    }

    // if ..(len/2) and (len/2).. are the same, continue
    let len_h = (len / 2);
    let left = &chars[..len_h];
    let right = &chars[len_h..];

    return *left != *right;
}

/// s == s[0:d] repeated n/d times
fn is_valid_id_part2(n: &str) -> bool {
    let bytes = n.as_bytes();
    let len = bytes.len();

    // we now need to find all denominators i guess because i can match three four and five times
    // so len/2 can match, but also len/3 len/4 .. len/n (1)
    for d in 1..len {
        // this can NEVER match since we can't get d chunks of the string
        // e.g. 1231234 we cannot get a left,right slice because it's uneven
        // this is fine to skip
        if len % d != 0 {
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
