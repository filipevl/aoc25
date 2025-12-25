use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let input = fs::read_to_string("inputs/day_09.txt").unwrap();
    //     let input = r#""

    let points: Vec<Point> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut s = l.split(',');
            Point {
                x: s.next().unwrap().trim().parse().unwrap(),
                y: s.next().unwrap().trim().parse().unwrap(),
            }
        })
        .collect();

    println!("Part 1: {}", part1(&points));
    println!("Part 2: {}", part2(&points));
}

fn part1(points: &[Point]) -> u64 {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| {
            points[i + 1..]
                .iter()
                .map(move |p2| (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1))
        })
        .max()
        .unwrap()
}

fn part2(points: &[Point]) -> u64 {
    let mut max_area = 0;

    for i in 0..points.len() {
        'points_loop: for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];

            let area = (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1);
            if area <= max_area {
                continue;
            }

            let x_min = p1.x.min(p2.x);
            let x_max = p1.x.max(p2.x);
            let y_min = p1.y.min(p2.y);
            let y_max = p1.y.max(p2.y);

            for i in 0..points.len() {
                let p1 = points[i];
                let p2 = points[(i + 1) % points.len()];

                if p1.x == p2.x {
                    if p1.x > x_min
                        && p1.x < x_max
                        && p1.y.min(p2.y) < y_max
                        && p1.y.max(p2.y) > y_min
                    {
                        continue 'points_loop;
                    }
                } else if p1.y > y_min
                    && p1.y < y_max
                    && p1.x.min(p2.x) < x_max
                    && p1.x.max(p2.x) > x_min
                {
                    continue 'points_loop;
                }
            }

            max_area = area;
        }
    }

    max_area
}
