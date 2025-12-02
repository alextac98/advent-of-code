#!/bin/bash

# Advent of Code Day Generator Script
# Generates Cargo project directories for each day of Advent of Code

set -e

# Default values
YEAR=$(date +%Y)
TOTAL_DAYS=25
BASE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Help message
show_help() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Generate Advent of Code Cargo project directories."
    echo ""
    echo "Options:"
    echo "  -y, --year YEAR       Year for Advent of Code (default: current year)"
    echo "  -d, --days DAYS       Total number of days to generate (default: 25)"
    echo "  -h, --help            Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                    # Generate all 25 days for current year"
    echo "  $0 -y 2024 -d 10      # Generate days 1-10 for 2024"
    echo "  $0 --year 2025        # Generate all 25 days for 2025"
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -y|--year)
            YEAR="$2"
            shift 2
            ;;
        -d|--days)
            TOTAL_DAYS="$2"
            shift 2
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Validate inputs
if ! [[ "$YEAR" =~ ^[0-9]{4}$ ]]; then
    echo "Error: Year must be a 4-digit number"
    exit 1
fi

if ! [[ "$TOTAL_DAYS" =~ ^[0-9]+$ ]] || [ "$TOTAL_DAYS" -lt 1 ] || [ "$TOTAL_DAYS" -gt 25 ]; then
    echo "Error: Days must be a number between 1 and 25"
    exit 1
fi

echo "🎄 Advent of Code $YEAR - Generating $TOTAL_DAYS day(s)"
echo "=================================================="

# Create year directory if it doesn't exist
YEAR_DIR="$BASE_DIR/$YEAR"
if [ ! -d "$YEAR_DIR" ]; then
    mkdir -p "$YEAR_DIR"
    echo "📁 Created year directory: $YEAR/"
fi

# Generate each day
created_count=0
skipped_count=0

for day in $(seq 1 $TOTAL_DAYS); do
    # Zero-pad the day number
    DAY_PADDED=$(printf "%02d" $day)
    DAY_DIR="$YEAR_DIR/$DAY_PADDED"
    
    if [ -d "$DAY_DIR" ]; then
        echo "⏭️  Day $DAY_PADDED: Already exists, skipping"
        ((skipped_count++))
        continue
    fi
    
    echo "🔨 Day $DAY_PADDED: Creating project..."
    
    # Create the directory
    mkdir -p "$DAY_DIR/src"
    
    # Create Cargo.toml
    cat > "$DAY_DIR/Cargo.toml" << EOF
[package]
name = "aoc-$YEAR-$DAY_PADDED"
version = "0.1.0"
edition = "2021"

[dependencies]
EOF
    
    # Create main.rs with a template
    cat > "$DAY_DIR/src/main.rs" << 'EOF'
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
EOF
    
    # Create empty input.txt
    touch "$DAY_DIR/input.txt"
    
    ((created_count++))
done

echo "=================================================="
echo "✅ Done!"
echo "   Created: $created_count day(s)"
echo "   Skipped: $skipped_count day(s) (already existed)"
