use std::fs;

fn get_input_path() -> &'static str {
    option_env!("AOC_INPUT_PATH").unwrap_or("input.txt")
}

fn max_joltage_k(bank: &str, k: usize) -> u64 {
    let digits: Vec<u64> = bank.chars().filter_map(|c| c.to_digit(10).map(|d| d as u64)).collect();
    let n = digits.len();

    if n < k {
        return 0;
    }

    let mut result: u64 = 0;
    let mut start = 0;

    for positions_remaining in (1..=k).rev() {
        let end = n - positions_remaining;

        let mut best_idx = start;
        let mut best_val = digits[start];
        for i in start..=end {
            if digits[i] > best_val {
                best_val = digits[i];
                best_idx = i;
            }
        }

        result = result * 10 + best_val;
        start = best_idx + 1;
    }

    result
}

fn part1(input: &str) -> u64 {
    return input
        .lines()
        .map(|line| max_joltage_k(line, 2))
        .sum();
}

fn part2(input: &str) -> u64 {
    return input
        .lines()
        .map(|line| max_joltage_k(line, 12))
        .sum();
}

fn main() {
    let input = fs::read_to_string(get_input_path()).expect("Failed to read input file");

    println!("Part 1: {}", part1(&input));
    
    println!("Part 2: {}", part2(&input));
}