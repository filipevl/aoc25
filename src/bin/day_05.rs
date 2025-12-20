use std::collections::HashSet;
use std::fs;
use std::ops::RangeInclusive;

enum ParsingMode {
    Fresh,
    Available,
}

fn main() {
    let input = fs::read_to_string("inputs/day_05.txt").expect("Failed to read input");
    //     let input = r#"3-5
    // 10-14
    // 16-20
    // 12-18
    //
    // 1
    // 5
    // 8
    // 11
    // 17
    // 32"#;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut count = 0;
    let mut parsing_mode = ParsingMode::Fresh;
    let mut fresh_ingredients = HashSet::new();

    'line_loop: for line in input.lines() {
        let line = line.trim();

        match parsing_mode {
            ParsingMode::Fresh => {
                if line.is_empty() {
                    parsing_mode = ParsingMode::Available;
                    continue;
                }

                let (lower, upper) = line.split_once("-").unwrap();
                let lower: usize = lower.parse().unwrap();
                let upper: usize = upper.parse().unwrap();
                let range = lower..=upper;

                fresh_ingredients.insert(range);
            }
            ParsingMode::Available => {
                let id = line.parse::<usize>().unwrap();
                for range in &fresh_ingredients {
                    if range.contains(&id) {
                        // println!("range: {}..={}, id: {}", range.start(), range.end(), id);
                        count += 1;
                        continue 'line_loop;
                    }
                }
            }
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let mut ranges: Vec<_> = input
        .lines()
        .map(|line| line.trim())
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (l, u) = line.split_once("-").unwrap();
            l.parse::<usize>().unwrap()..=u.parse::<usize>().unwrap()
        })
        .collect();

    ranges.sort_unstable_by_key(|r| *r.start());

    let merged = ranges
        .into_iter()
        .fold(Vec::<RangeInclusive<usize>>::new(), |mut acc, next| {
            match acc.last_mut() {
                Some(last) if *next.start() <= *last.end() + 1 => {
                    let new_end = (*last.end()).max(*next.end());
                    *last = *last.start()..=new_end;
                }
                _ => acc.push(next),
            }
            acc
        });

    merged.iter().map(|r| (r.end() - r.start()) + 1).sum()
}
