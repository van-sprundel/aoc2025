pub fn part1(input: &str) {
    solution(input, Part::PART1)
}
pub fn part2(input: &str) {
    solution(input, Part::PART2)
}

fn solution(input: &str, part: Part) {
    let mut invalid_id_acc = 0;
    input.split(',').for_each(|range| {
        range.find('-').iter().for_each(|&index| {
            let left = range[..index].trim().parse::<usize>().unwrap();
            let right = range[index + 1..].trim().parse::<usize>().unwrap();

            invalid_id_acc += (left..right)
                .map(|i| (!is_valid_id(&i.to_string(), part) as usize) * i)
                .sum::<usize>()
        });
    });

    println!("{invalid_id_acc}")
}

// enums arent real xd
#[derive(PartialEq, Copy, Clone)]
enum Part {
    PART1 = 0,
    PART2 = 1,
}

fn is_valid_id(n: &str, part: Part) -> bool {
    let bytes = n.as_bytes();
    let len = bytes.len();

    // we now need to find all denominators i guess because i can match three four and five times
    // so len/2 can match, but also len/3 len/4 .. len/n (1)
    /// s(part1) == s[0:2] repeated n/2 times
    /// s(part2) == s[0:d] repeated n/d times
    // this means that part1 only needs to iter on d..d+1 and it will be sufficient
    // for part2 we need to iter 1..d+1
    // both ends can be d+1 because we can't surpass d (we can't make less than 2 chunks)
    let d = len / 2;
    let is_part1 = (part as u8) ^ 1;
    let is_part2 = part as u8;

    // part1: start_d = d,
    // part2: start_d = 1
    let start_d = is_part1 as usize * d + is_part2 as usize;
    let end_d = d + 1;

    // part 1 has a specific condition where if the length doesnt match we continue
    // this also applies for part 2 but for chunk d
    // for some reason len.is_multiple_of doesn't cover this
    let part1_invalid = is_part1 as usize & (len & 1);

    let mut all_divisors_invalid = 1usize;
    for divisor in start_d..end_d {
        // this can NEVER match since we can't get d chunks of the string
        // e.g. 1231234 we cannot get a left,right slice because it's uneven
        // this is fine to skip
        let not_multiple = (!len.is_multiple_of(divisor)) as usize;

        // then we can do the actual logic where we check if any chunk does NOT match the expected chunk
        let mut any_mismatch = 0usize;
        let mut i = divisor;
        while divisor > 0 && i + divisor <= len {
            let mut chunk_mismatch = 0usize;
            for j in 0..divisor {
                chunk_mismatch |= (bytes[i + j] ^ bytes[j]) as usize;
            }
            any_mismatch |= (chunk_mismatch != 0) as usize;
            i += divisor;
        }

        all_divisors_invalid &= not_multiple | any_mismatch;
    }

    (part1_invalid | all_divisors_invalid) != 0
}
