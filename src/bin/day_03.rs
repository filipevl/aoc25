use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day_03.txt").expect("Failed to read input");
    //     let input = r#"987654321111111
    // 811111111111119
    // 234234234234278
    // 818181911112111"#;

    let banks: Vec<&str> = input.trim().lines().collect();

    println!("Part 1: {}", part1(&banks));
    println!("Part 2: {}", part2(&banks));
}

fn part1(banks: &[&str]) -> u64 {
    let mut sum = 0;

    for bank in banks {
        let mut most_significant_joltage = 0;
        let mut least_significant_joltage = 0;

        for (i, c) in bank.chars().enumerate() {
            // println!("index: {}, char: {}", i, c);
            let digit: u64 = c.to_digit(10).unwrap() as u64;

            if digit > most_significant_joltage && i != bank.len() - 1 {
                most_significant_joltage = digit;
                least_significant_joltage = 0;
                continue;
            }

            if digit > least_significant_joltage {
                least_significant_joltage = digit;
                continue;
            }
        }

        // println!(
        //     "Most significant joltage: {}, Least significant joltage: {}",
        //     most_significant_joltage, least_significant_joltage
        // );
        sum += most_significant_joltage * 10 + least_significant_joltage;
    }

    sum
}

fn part2(banks: &[&str]) -> u64 {
    let mut sum = 0;

    for bank in banks {
        let mut joltage: [u64; 12] = [0; 12];

        'char_loop: for (i, c) in bank.chars().enumerate() {
            let remaining_batteries = bank.len() - 1 - i;
            let joltage_start_index = (joltage.len() - 1).saturating_sub(remaining_batteries);

            let digit: u64 = c.to_digit(10).unwrap() as u64;

            // println!(
            //     "bank index: {}, battery: {}, remaining_batteries: {}, joltage_start_index: {}",
            //     i, c, remaining_batteries, joltage_start_index
            // );

            for j in joltage_start_index..joltage.len() {
                if digit > joltage[j] {
                    joltage[j] = digit;
                    joltage[j + 1..].fill(0);
                    continue 'char_loop;
                }
            }
        }

        // construct a number from all digits
        let mut number: u64 = 0;
        for &digit in joltage.iter() {
            number = number * 10 + digit;
        }

        // println!("Number: {}", number);
        sum += number;
    }

    sum
}
