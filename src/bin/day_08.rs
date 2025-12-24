use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    num_sets: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            num_sets: n,
        }
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return false;
        }

        let (major, minor) = if self.size[root_a] >= self.size[root_b] {
            (root_a, root_b)
        } else {
            (root_b, root_a)
        };

        self.parent[minor] = major;
        self.size[major] += self.size[minor];
        self.num_sets -= 1;

        true
    }

    pub fn num_sets(&self) -> usize {
        self.num_sets
    }
}

fn main() {
    let input = fs::read_to_string("inputs/day_08.txt").expect("Failed to read input");
    //     let input = r#"162,817,812
    // 57,618,57
    // 906,360,560
    // 592,479,940
    // 352,342,300
    // 466,668,158
    // 542,29,236
    // 431,825,988
    // 739,650,466
    // 52,470,668
    // 216,146,977
    // 819,987,18
    // 117,168,530
    // 805,96,715
    // 346,949,466
    // 970,615,88
    // 941,993,340
    // 862,61,35
    // 984,92,344
    // 425,690,689
    // "#;

    let points: Vec<(u32, u32, u32)> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<u32>().unwrap();
            let y = parts.next().unwrap().parse::<u32>().unwrap();
            let z = parts.next().unwrap().parse::<u32>().unwrap();
            (x, y, z)
        })
        .collect();

    println!("Part 1: {}", part1(&points));
    println!("Part 2: {}", part2(&points));
}

fn part1(points: &[(u32, u32, u32)]) -> usize {
    let n = points.len();

    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points[i].0 as i64 - points[j].0 as i64;
            let dy = points[i].1 as i64 - points[j].1 as i64;
            let dz = points[i].2 as i64 - points[j].2 as i64;
            let dist2 = dx * dx + dy * dy + dz * dz;
            edges.push((dist2, i, j));
        }
    }

    edges.sort_unstable_by_key(|e| e.0);

    let mut uf = UnionFind::new(n);

    let mut processed = 0;
    while processed < 1000 {
        uf.union(edges[processed].1, edges[processed].2);
        processed += 1;
    }

    let mut components: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        let root = uf.find(i);
        *components.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = components.values().cloned().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    sizes[0] * sizes[1] * sizes[2]
}

fn part2(points: &[(u32, u32, u32)]) -> u64 {
    let n = points.len();
    if n < 2 {
        return 0;
    } // Handle edge case explicitly

    let mut edges: Vec<(i64, usize, usize)> = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points[i].0 as i64 - points[j].0 as i64;
            let dy = points[i].1 as i64 - points[j].1 as i64;
            let dz = points[i].2 as i64 - points[j].2 as i64;
            edges.push((dx * dx + dy * dy + dz * dz, i, j));
        }
    }

    edges.sort_unstable_by_key(|e| e.0);

    let mut uf = UnionFind::new(n);

    for (dist2, i, j) in edges {
        if uf.union(i, j) && uf.num_sets() == 1 {
            return points[i].0 as u64 * points[j].0 as u64;
        }
    }

    panic!("Graph was not connected; no result found")
}
