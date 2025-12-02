use std::fs;

fn get_input_path() -> &'static str {
    option_env!("AOC_INPUT_PATH").unwrap_or("input.txt")
}

fn main() {
    let input = fs::read_to_string(get_input_path()).expect("Failed to read input file");

    // TODO: Implement solution
    println!("{}", input);
}
