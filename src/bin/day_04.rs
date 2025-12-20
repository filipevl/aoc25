use std::collections::VecDeque;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day_04.txt").expect("Failed to read input");
    //     let input = r#"..@@.@@@@.
    // @@@.@.@.@@
    // @@@@@.@.@@
    // @.@@@@..@.
    // @@.@@@@.@@
    // .@@@@@@@.@
    // .@.@.@.@@@
    // @.@@@.@@@@
    // .@@@@@@@@.
    // @.@.@@@.@.
    // "#;

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(grid));
}

fn part1(grid: &[Vec<char>]) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for y in 0..height {
        for x in 0..width {
            if grid[y][x] != '@' {
                continue;
            }

            let surrounding_papers = directions
                .iter()
                .filter_map(|(dy, dx)| {
                    let ny = y as isize + dy;
                    let nx = x as isize + dx;

                    if ny >= 0 && nx >= 0 && (ny as usize) < height && (nx as usize) < width {
                        Some(grid[ny as usize][nx as usize])
                    } else {
                        None
                    }
                })
                .filter(|&ch| ch == '@')
                .count();

            if surrounding_papers < 4 {
                count += 1;
            }
        }
    }

    count
}

fn part2(mut grid: Vec<Vec<char>>) -> usize {
    let h = grid.len();
    let w = grid[0].len();

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut neighbors = vec![vec![0u8; w]; h];

    for y in 0..h {
        for x in 0..w {
            if grid[y][x] != '@' {
                continue;
            }

            for (dy, dx) in directions {
                let ny = y as isize + dy;
                let nx = x as isize + dx;
                if ny >= 0
                    && nx >= 0
                    && (ny as usize) < h
                    && (nx as usize) < w
                    && grid[ny as usize][nx as usize] == '@'
                {
                    neighbors[y][x] += 1;
                }
            }
        }
    }

    let mut queue = VecDeque::new();

    for y in 0..h {
        for x in 0..w {
            if grid[y][x] == '@' && neighbors[y][x] < 4 {
                queue.push_back((x, y));
            }
        }
    }

    let mut removed = 0;

    while let Some((x, y)) = queue.pop_front() {
        grid[y][x] = '.';
        removed += 1;

        // Update neighbors
        for (dy, dx) in directions {
            let ny = y as isize + dy;
            let nx = x as isize + dx;

            if ny >= 0
                && nx >= 0
                && (ny as usize) < h
                && (nx as usize) < w
                && grid[ny as usize][nx as usize] == '@'
            {
                neighbors[ny as usize][nx as usize] -= 1;

                // Just crossed below threshold
                if neighbors[ny as usize][nx as usize] == 3 {
                    queue.push_back((nx as usize, ny as usize));
                }
            }
        }
    }

    removed
}
