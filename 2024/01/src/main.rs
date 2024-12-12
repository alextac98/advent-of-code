use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {

    // Specify the file path
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();

    // Process each line
    for line in reader.lines() {
        let line = line?; // Unwrap the Result

        // Split the line by three spaces
        let parts: Vec<&str> = line.split("   ").collect();

        if parts.len() == 2 {
            // Parse the numbers and push them to respective vectors
            if let Ok(first) = parts[0].trim().parse::<i32>() {
                first_numbers.push(first);
            }
            if let Ok(second) = parts[1].trim().parse::<i32>() {
                second_numbers.push(second);
            }
        }
    }

    // Sort the vectors
    first_numbers.sort();
    second_numbers.sort();

    // Subtract vector elements
    let differences: Vec<i32> = first_numbers
        .iter()
        .zip(second_numbers.iter())
        .map(|(f, s)| (f - s).abs())
        .collect();

    let total_sum: i32 = differences.iter().sum();

    // Print the sorted vectors and their differences
    // println!("First numbers: {:?}", first_numbers);
    // println!("Second numbers: {:?}", second_numbers);
    // println!("Differences: {:?}", differences);

    println!("Total Distances: {:?}", total_sum);

    Ok(())
}
