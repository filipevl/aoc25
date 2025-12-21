use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day_07.txt").expect("Failed to read input");
    //     let input = r#".......S.......
    // ...............
    // .......^.......
    // ...............
    // ......^.^......
    // ...............
    // .....^.^.^.....
    // ...............
    // ....^.^...^....
    // ...............
    // ...^.^...^.^...
    // ...............
    // ..^...^.....^..
    // ...............
    // .^.^.^.^.^...^.
    // ...............
    // "#;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut split_count = 0;

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut current_beams = HashSet::new();
    current_beams.insert(grid[0].iter().position(|&c| c == 'S').unwrap());

    for line in grid.iter().skip(1) {
        let mut next_beams = HashSet::new();

        for x in current_beams {
            if line[x] == '^' {
                split_count += 1;

                next_beams.insert(x - 1);
                next_beams.insert(x + 1);
            } else {
                next_beams.insert(x);
            }
        }

        current_beams = next_beams;
    }

    split_count
}

fn part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let width = grid[0].len();

    let mut ways = vec![0usize; width];
    ways[grid[0].iter().position(|&c| c == 'S').unwrap()] = 1;

    for row in grid.iter().skip(1) {
        let mut next_ways = vec![0usize; width];

        for x in 0..width {
            let count = ways[x];
            if count == 0 {
                continue;
            }

            if row[x] == '^' {
                next_ways[x - 1] += count;
                next_ways[x + 1] += count;
            } else {
                next_ways[x] += count;
            }
        }
        ways = next_ways;
    }

    ways.iter().sum()
}
