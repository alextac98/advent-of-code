use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    
    let part1 = solve_part1(&input);
    println!("Part 1: {}", part1);
    
    let part2 = solve_part2(&input);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> i64 {
    // TODO: Implement part 1
    0
}

fn solve_part2(input: &str) -> i64 {
    // TODO: Implement part 2
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const EXAMPLE: &str = r#"
"#;
    
    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(EXAMPLE.trim()), 0);
    }
    
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(EXAMPLE.trim()), 0);
    }
}
