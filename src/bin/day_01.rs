use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day_01.txt").expect("Failed to read input");
    //     let input = r#"L68
    // L30
    // R48
    // L5
    // R60
    // L55
    // L1
    // L99
    // R14
    // L82
    // "#;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut count = 0;

    let mut dial = 50;
    for line in input.lines() {
        let (prefix, rest) = line.split_at(1);
        let mut number: i32 = rest.parse().unwrap();
        if prefix == "L" {
            number = -number;
        }

        dial = (dial + number) % 100;

        if dial == 0 {
            count += 1;
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let mut total_count = 0;
    let mut dial = 50;

    for line in input.lines() {
        let line = line.trim();

        let direction = &line[0..1];
        let distance: i32 = line[1..].parse().unwrap();

        let hits = if direction == "L" {
            if dial == 0 {
                distance / 100
            } else if distance >= dial {
                1 + (distance - dial) / 100
            } else {
                0
            }
        } else {
            if dial == 0 {
                distance / 100
            } else {
                let clicks_to_first_zero = 100 - dial;
                if distance >= clicks_to_first_zero {
                    1 + (distance - clicks_to_first_zero) / 100
                } else {
                    0
                }
            }
        };

        total_count += hits as usize;

        let delta = if direction == "L" {
            -distance
        } else {
            distance
        };
        
        dial = (dial + delta).rem_euclid(100);
    }

    total_count
}
