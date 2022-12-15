//! The module providing [main].

/// A declarative macro for creating the main function of an aoc puzzle.
#[macro_export]
macro_rules! main {
    ($package:ident) => {{
        use utils::output::Output;

        let input_dir = format!("{}/input", std::env!("CARGO_MANIFEST_DIR"));

        let input_file_name = match std::env::args().nth(1) {
            Some(arg) => arg,
            None => String::from("input"),
        };

        let input_file_path = format!("{input_dir}/{input_file_name}.txt");
        let input = std::fs::read_to_string(input_file_path).expect("failed to read input file");

        println!("part1: {}", $package::part1(&input).output());
        println!("part2: {}", $package::part2(&input).output());
    }};
}
