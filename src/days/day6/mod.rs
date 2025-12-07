pub fn part1(input: &str) {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        println!("0");
        return;
    }

    // find the width
    let width = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    // find boundaries (using width)
    let mut is_separator = vec![true; width];

    for line in &lines {
        for (i, ch) in line.chars().enumerate() {
            if ch != ' ' {
                is_separator[i] = false;
            }
        }
    }

    // find operation start
    // (columns that are not separators)
    let mut problem_columns = Vec::new();
    let mut in_problem = false;

    for (i, item) in is_separator.iter().enumerate().take(width) {
        if !item && !in_problem {
            problem_columns.push(i);
            in_problem = true;
        } else if is_separator[i] {
            in_problem = false;
        }
    }

    // extract numbers and operation
    let mut grand_total: i64 = 0;

    for &start_col in &problem_columns {
        let end_col = (start_col..width)
            .find(|&i| is_separator[i])
            .unwrap_or(width);

        let mut numbers = Vec::new();
        let mut operation = '+';

        for line in &lines {
            if line.len() <= start_col {
                continue;
            }

            let segment = if line.len() >= end_col {
                &line[start_col..end_col]
            } else {
                &line[start_col..]
            };

            let trimmed = segment.trim();

            if trimmed.is_empty() {
                continue;
            }

            if trimmed == "+" || trimmed == "*" {
                operation = trimmed.chars().next().unwrap();
            } else if let Ok(num) = trimmed.parse::<i64>() {
                numbers.push(num);
            }
        }

        if !numbers.is_empty() {
            let result: i64 = if operation == '+' {
                numbers.iter().sum()
            } else {
                numbers.iter().product()
            };
            grand_total += result;
        }
    }

    println!("{}", grand_total);
}

pub fn part2(input: &str) {
    todo!("Day 6 part 2 not implemented yet");
}
