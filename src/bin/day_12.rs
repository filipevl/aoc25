use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day_12.txt").expect("Failed to read input");
    println!("Part 1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let sections: Vec<&str> = input.trim().split("\n\n").collect();

    let mut shape_areas = Vec::new();
    for block in sections[0..sections.len() - 1].iter() {
        let area = block.chars().filter(|&c| c == '#').count();
        shape_areas.push(area);
    }

    let mut total_fits = 0;
    let regions_section = sections.last().unwrap();

    for line in regions_section.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let dims: Vec<usize> = parts[0]
            .split('x')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        let counts: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (width, height) = (dims[0], dims[1]);
        let region_area = width * height;

        let total_present_area: usize = counts
            .iter()
            .enumerate()
            .map(|(i, &qty)| qty * shape_areas[i])
            .sum();

        if total_present_area <= region_area {
            total_fits += 1;
        }
    }

    total_fits
}
