use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day_11.txt").expect("Failed to read input");
    //     let input = r#"aaa: you hhh
    // you: bbb ccc
    // bbb: ddd eee
    // ccc: ddd eee fff
    // ddd: ggg
    // eee: out
    // fff: out
    // ggg: out
    // hhh: ccc fff iii
    // iii: out
    // "#;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.trim().lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let name = parts[0];
        let connections: Vec<&str> = parts[1].split_whitespace().collect();
        graph.insert(name, connections);
    }

    let mut memo: HashMap<&str, usize> = HashMap::new();

    part1_count_paths("you", &graph, &mut memo)
}

fn part1_count_paths<'a>(
    current: &'a str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if current == "out" {
        return 1;
    }

    if let Some(&count) = memo.get(current) {
        return count;
    }

    let mut total = 0;
    if let Some(outputs) = graph.get(current) {
        for output in outputs {
            total += part1_count_paths(output, graph, memo);
        }
    }

    memo.insert(current, total);
    total
}

fn part2(input: &str) -> usize {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.trim().lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let name = parts[0];
        let connections: Vec<&str> = parts[1].split_whitespace().collect();
        graph.insert(name, connections);
    }

    let mut memo: HashMap<(&str, u8), usize> = HashMap::new();

    part2_count_paths("svr", 0, &graph, &mut memo)
}

fn part2_count_paths<'a>(
    current: &'a str,
    mut mask: u8,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<(&'a str, u8), usize>,
) -> usize {
    if current == "fft" {
        mask |= 1;
    }

    if current == "dac" {
        mask |= 2;
    }

    if current == "out" {
        return if mask == 3 { 1 } else { 0 };
    }

    if let Some(&count) = memo.get(&(current, mask)) {
        return count;
    }

    let mut total = 0;
    if let Some(neighbors) = graph.get(current) {
        for &neighbor in neighbors {
            total += part2_count_paths(neighbor, mask, graph, memo);
        }
    }

    memo.insert((current, mask), total);
    total
}
