use std::fs;

fn get_input_path() -> &'static str {
    option_env!("AOC_INPUT_PATH").unwrap_or("input_final.txt")
}

fn generate_grid(input: &str) -> Vec<Vec<char>> {
    return input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
}

fn count_adjacent_rolls(grid: &[Vec<char>], row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    // Check all 8 adjacent positions
    for dr in -1i32..=1 {
        for dc in -1i32..=1 {
            // Skip the center cell
            if dr == 0 && dc == 0 {
                continue;
            }

            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            // Check bounds
            if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                if grid[new_row as usize][new_col as usize] == '@' {
                    count += 1;
                }
            }
        }
    }

    return count;
}

fn find_accessible_rolls(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut accessible = Vec::new();

    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            // Only check cells that have a roll of paper
            if ch == '@' {
                let adjacent = count_adjacent_rolls(grid, row, col);
                // Forklift can access if fewer than 4 adjacent rolls
                if adjacent < 4 {
                    accessible.push((row, col));
                }
            }
        }
    }

    accessible
}

fn part1(grid: &[Vec<char>]) -> usize {
    find_accessible_rolls(grid).len()
}

fn part2(grid: &[Vec<char>]) -> usize {
    let mut grid = grid.to_vec();
    let mut total_removed = 0;

    loop {
        let accessible = find_accessible_rolls(&grid);

        if accessible.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (row, col) in &accessible {
            grid[*row][*col] = '.';
        }

        total_removed += accessible.len();
    }

    total_removed
}

fn main() {
    let input = fs::read_to_string(get_input_path()).expect("Failed to read input file");

    let grid = generate_grid(&input);
    println!("Generated grid with {} rows and {} columns", grid.len(), grid[0].len());

    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}
