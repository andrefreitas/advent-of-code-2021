use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_puzzle_from_file<P: AsRef<Path>>(filename: P) -> Vec<u32> {
    let mut puzzle: Vec<u32> = Vec::new();
    let file = File::open(filename).expect("File doesn't exist");
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(depth) = line {
            puzzle.push(depth.parse().unwrap());
        }
    }
    return puzzle;
}

fn count_increments(puzzle: &Vec<u32>) -> u32 {
    let mut increments = 0;
    let mut previous_depth: &u32 = puzzle.first().unwrap();

    for depth in puzzle.iter() {
        if depth > previous_depth {
            increments += 1;
        }
        previous_depth = depth;
    }
    return increments;
}

fn group_in_sliding_windows(puzzle: &Vec<u32>, window: usize) -> Vec<u32> {
    return (0..=puzzle.len() - window)
        .map(|index| puzzle[index..index + window].iter().sum())
        .collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let puzzle: Vec<u32> = read_puzzle_from_file(filename);
    println!("Parsed {} with {} lines", filename, puzzle.len());

    let total_increments: u32 = count_increments(&puzzle);
    println!("Total increments: {}", total_increments);

    let sliding_windows: Vec<u32> = group_in_sliding_windows(&puzzle, 3);
    let sliding_window_increments = count_increments(&sliding_windows);
    println!("Sliding window increments: {}", sliding_window_increments);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_increments_when_0() {
        assert_eq!(count_increments(&vec![1]), 0);
    }

    #[test]
    fn test_count_increments_when_exists() {
        assert_eq!(
            count_increments(&vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        );
    }
}
