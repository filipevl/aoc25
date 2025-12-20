use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day_02.txt").expect("Failed to read input");
    // let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    let input: Vec<&str> = input.trim().split(',').collect();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(id_ranges: &[&str]) -> u64 {
    let mut count = 0;
    let mut sum = 0;

    for range in id_ranges {
        let (lower, higher) = range.split_once('-').expect("invalid range");

        // println!("Trying to find pair: {}-{}", lower, higher);

        let lower: u64 = lower.parse().unwrap();
        let higher: u64 = higher.parse().unwrap();

        for id in lower..=higher {
            let id_str = id.to_string();
            let id_len = id_str.len();
            if id_len % 2 == 0 {
                let (left, right) = id_str.as_str().split_at(id_len / 2);
                if left == right {
                    count += 1;
                    sum += id;
                    println!("Found pair: {} -> sum: {}, count: {}", id_str, sum, count);
                }
            }
        }
    }

    sum
}

fn part2(id_ranges: &[&str]) -> u64 {
    let mut count = 0;
    let mut sum = 0;

    for range in id_ranges {
        let (lower, higher) = range.split_once('-').expect("invalid range");

        // println!("Trying to find pair: {}-{}", lower, higher);

        let lower: u64 = lower.parse().unwrap();
        let higher: u64 = higher.parse().unwrap();

        'id_loop: for id in lower..=higher {
            let id_str = id.to_string();
            let total_len = id_str.len();

            for prefix_len in 1..=total_len / 2 {
                if total_len % prefix_len != 0 {
                    continue;
                }

                let pattern = &id_str[..prefix_len];
                let repetitions = total_len / prefix_len;
                if pattern.repeat(repetitions) == id_str {
                    count += 1;
                    sum += id;
                    println!("Found pair: {} -> sum: {}, count: {}", id_str, sum, count);
                    continue 'id_loop;
                }
            }
        }
    }

    sum
}
