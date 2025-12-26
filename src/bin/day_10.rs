use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day_10.txt").expect("Failed to read input");
    // let input = r#""#;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2());
}

fn part1(input: &str) -> u32 {
    let mut total_presses = 0;

    for line in input.lines().filter(|l| !l.is_empty()) {
        let start_bracket = line.find('[').unwrap();
        let end_bracket = line.find(']').unwrap();
        let target_str = &line[start_bracket + 1..end_bracket];

        let mut target_mask: u32 = 0;
        for (i, c) in target_str.chars().enumerate() {
            if c == '#' {
                target_mask |= 1 << i;
            }
        }

        let mut buttons = Vec::new();
        let mut cursor = end_bracket + 1;
        while let Some(start_paren) = line[cursor..].find('(') {
            let actual_start = cursor + start_paren;
            let actual_end = actual_start + line[actual_start..].find(')').unwrap();

            let mut btn_mask: u32 = 0;
            for part in line[actual_start + 1..actual_end].split(',') {
                if let Ok(bit) = part.trim().parse::<u32>() {
                    btn_mask |= 1 << bit;
                }
            }
            buttons.push(btn_mask);
            cursor = actual_end + 1;
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((0u32, 0u32));
        visited.insert(0u32);

        while let Some((state, dist)) = queue.pop_front() {
            if state == target_mask {
                total_presses += dist;
                break;
            }

            for &btn in &buttons {
                let next_state = state ^ btn;
                if !visited.contains(&next_state) {
                    visited.insert(next_state);
                    queue.push_back((next_state, dist + 1));
                }
            }
        }
    }

    total_presses
}

fn part2() -> usize {
    0
}
