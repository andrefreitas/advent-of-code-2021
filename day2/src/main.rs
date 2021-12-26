use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

struct Movement {
    direction: Direction,
    amount: u32,
}

fn read_direction(direction: &str) -> Option<Direction> {
    return match direction {
        "up" => Some(Direction::Up),
        "down" => Some(Direction::Down),
        "forward" => Some(Direction::Forward),
        &_ => None,
    };
}

fn read_puzzle_from_file<P: AsRef<Path>>(filename: P) -> Vec<Movement> {
    let mut puzzle: Vec<Movement> = Vec::new();
    let file = File::open(filename).expect("File doesn't exist");
    let lines = io::BufReader::new(file).lines();
    for line in lines {
        if let Ok(instruction) = line {
            let instructions: Vec<&str> = instruction.split(" ").collect();
            let direction = read_direction(instructions[0]).unwrap();
            let amount: u32 = instructions[1].parse().unwrap();
            puzzle.push(Movement { direction, amount });
        }
    }
    return puzzle;
}

fn compute_final_position(puzzle: &Vec<Movement>) -> (u32, u32) {
    let mut depth: u32 = 0;
    let mut horizontal: u32 = 0;

    for movement in puzzle.iter() {
        match movement.direction {
            Direction::Up => depth -= movement.amount,
            Direction::Down => depth += movement.amount,
            Direction::Forward => horizontal += movement.amount,
        }
    }

    return (depth, horizontal);
}

fn compute_final_position_with_aim(puzzle: &Vec<Movement>) -> (u32, u32) {
    let mut depth: u32 = 0;
    let mut horizontal: u32 = 0;
    let mut aim: u32 = 0;

    for movement in puzzle.iter() {
        match movement.direction {
            Direction::Up => aim -= movement.amount,
            Direction::Down => aim += movement.amount,
            Direction::Forward => {
                horizontal += movement.amount;
                depth += aim * movement.amount
            }
        }
    }

    return (depth, horizontal);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let puzzle = read_puzzle_from_file(filename);
    println!("Read puzzle {} with {} movements", filename, puzzle.len());

    let (depth, horizontal) = compute_final_position(&puzzle);
    println!(
        "Final position is {} (depth), {} (horizontal) which is {}",
        depth,
        horizontal,
        depth * horizontal
    );

    let (depth, horizontal) = compute_final_position_with_aim(&puzzle);
    println!(
        "Final position with aim is {} (depth), {} (horizontal) which is {}",
        depth,
        horizontal,
        depth * horizontal
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_easy_puzzle() -> Vec<Movement> {
        return read_puzzle_from_file("input-easy.txt");
    }

    #[test]
    fn test_compute_final_position() {
        let puzzle = read_easy_puzzle();
        assert_eq!(compute_final_position(&puzzle), (10, 15));
    }

    #[test]
    fn test_compute_final_position_with_aim() {
        let puzzle = read_easy_puzzle();
        assert_eq!(compute_final_position_with_aim(&puzzle), (60, 15));
    }
}
