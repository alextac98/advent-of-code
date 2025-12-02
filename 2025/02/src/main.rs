use std::fs;

/// Get the input file path - uses Bazel runfiles or falls back to current directory
fn get_input_path() -> &'static str {
    option_env!("AOC_INPUT_PATH").unwrap_or("input.txt")
}

/// Check if a number is "invalid" - made of some sequence of digits repeated at least twice
/// e.g., 55 (5 twice), 6464 (64 twice), 123123123 (123 three times), 1111111 (1 seven times)
fn is_invalid_id(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    
    // Try all possible pattern lengths that could repeat at least twice
    // Pattern length must divide evenly into total length, and we need at least 2 repetitions
    for pattern_len in 1..=len/2 {
        if len % pattern_len != 0 {
            continue;
        }
        
        let pattern = &s[..pattern_len];
        let repetitions = len / pattern_len;
        
        // Check if the entire string is made of this pattern repeated
        let mut is_match = true;
        for i in 1..repetitions {
            let start = i * pattern_len;
            let end = start + pattern_len;
            if &s[start..end] != pattern {
                is_match = false;
                break;
            }
        }
        
        if is_match {
            return true;
        }
    }
    
    false
}

fn solve(input: &str) -> u64 {
    let mut total: u64 = 0;
    
    // Parse ranges separated by commas
    for range_str in input.trim().split(',') {
        let range_str = range_str.trim();
        if range_str.is_empty() {
            continue;
        }
        
        // Parse "start-end" format
        let parts: Vec<&str> = range_str.split('-').collect();
        if parts.len() != 2 {
            continue;
        }
        
        let start: u64 = parts[0].parse().expect("Invalid start number");
        let end: u64 = parts[1].parse().expect("Invalid end number");
        
        // Find all invalid IDs in this range
        for id in start..=end {
            if is_invalid_id(id) {
                total += id;
            }
        }
    }
    
    total
}

fn main() {
    // Read and solve the actual input
    let input = fs::read_to_string(get_input_path()).expect("Failed to read input.txt");
    let result = solve(&input);
    println!("Answer: {}", result);
}
