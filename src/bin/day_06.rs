use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day_06.txt").expect("Failed to read input");
    // let input = r#""#;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let n_rows = lines.len();

    let grid: Vec<Vec<u64>> = lines[..n_rows - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let ops: Vec<char> = lines[n_rows - 1]
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect();

    let mut grand_total = 0;
    for x in 0..ops.len() {
        let column_nums: Vec<u64> = grid.iter().map(|row| row[x]).collect();

        let result = match ops[x] {
            '+' => column_nums.iter().sum::<u64>(),
            '*' => column_nums.iter().product::<u64>(),
            _ => 0,
        };
        grand_total += result;
    }

    grand_total
}

fn part2(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let n_rows = lines.len();
    let width = lines[0].len();

    let mut total = 0;
    let mut current_numbers = Vec::new();

    for x in (0..width).rev() {
        let mut column_string = String::new();
        for line in lines.iter().take(n_rows - 1) {
            let c = line.chars().nth(x).unwrap_or(' ');
            if c.is_ascii_digit() {
                column_string.push(c);
            }
        }

        if !column_string.is_empty() {
            let num = column_string.parse::<u64>().unwrap();
            current_numbers.push(num);
        }

        let op_char = lines[n_rows - 1].chars().nth(x).unwrap_or(' ');

        if op_char == '*' || op_char == '+' {
            let result = if op_char == '*' {
                current_numbers.iter().product::<u64>()
            } else {
                current_numbers.iter().sum::<u64>()
            };

            total += result;
            current_numbers.clear();
        }
    }

    total
}
