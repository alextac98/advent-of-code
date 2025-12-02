use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    
    let mut position: i32 = 50;
    let mut zero_count_part1 = 0;
    let mut zero_count_part2 = 0;
    
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        let direction = &line[0..1];
        let distance: i32 = line[1..].parse().expect("Failed to parse distance");
        
        let old_position = position;
        
        match direction {
            "L" => position -= distance,
            "R" => position += distance,
            _ => panic!("Unknown direction: {}", direction),
        }
        
        let zeros_crossed = if direction == "L" {
            // Moving left: from old_position down to position (position < old_position)
            // Count multiples of 100 in [position, old_position)
            // i.e., k*100 where position <= k*100 < old_position
            let low = position;
            let high = old_position;
            let first_multiple = ((low as f64) / 100.0).ceil() as i32 * 100;
            let last_multiple = (((high - 1) as f64) / 100.0).floor() as i32 * 100;
            if first_multiple <= last_multiple {
                (last_multiple - first_multiple) / 100 + 1
            } else {
                0
            }
        } else {
            // Moving right: from old_position up to position (position > old_position)
            // Count multiples of 100 in (old_position, position]
            // i.e., k*100 where old_position < k*100 <= position
            let low = old_position;
            let high = position;
            let first_multiple = ((low as f64) / 100.0).floor() as i32 * 100 + 100;
            let last_multiple = ((high as f64) / 100.0).floor() as i32 * 100;
            if first_multiple <= last_multiple && first_multiple > low {
                (last_multiple - first_multiple) / 100 + 1
            } else {
                0
            }
        };
        
        zero_count_part2 += zeros_crossed;
        
        // Wrap around the dial (0-99)
        position = position.rem_euclid(100);
        
        if position == 0 {
            zero_count_part1 += 1;
        }
    }
    
    println!("Part 1 - Password (times dial ended at 0): {}", zero_count_part1);
    println!("Part 2 - Password (times dial clicked at 0): {}", zero_count_part2);
}
