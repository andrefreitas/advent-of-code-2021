use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct PowerConsumption {
    gamma_rate: u32,
    epsilon_rate: u32,
}

impl PowerConsumption {
    fn consumption(self) -> u32 {
        self.gamma_rate * self.epsilon_rate
    }
}

fn read_puzzle_from_file<P: AsRef<Path>>(filename: P) -> Vec<String> {
    let file = File::open(filename).expect("File doesn't exist");
    return io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
}

fn compute_power_consumption(puzzle: &Vec<String>) -> PowerConsumption {
    let mut power_consumption = PowerConsumption {
        gamma_rate: 0,
        epsilon_rate: 0,
    };

    let width = puzzle[0].len();
    for position in 0..width {
        let column: Vec<char> = puzzle
            .iter()
            .map(|number| number.chars().nth(position).unwrap())
            .collect();

        let ones = column.iter().filter(|&&bit| bit == '1');
        let zeros = column.iter().filter(|&&bit| bit == '0');
        let increment = 1 << (width - 1 - position);

        if ones.count() > zeros.count() {
            power_consumption.gamma_rate += increment;
        } else {
            power_consumption.epsilon_rate += increment;
        }
    }

    return power_consumption;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let puzzle = read_puzzle_from_file(filename);
    let power_consumption = compute_power_consumption(&puzzle);
    println!("Power consumption is {:?}", power_consumption.consumption());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_power_consumption_easy() {
        let puzzle = read_puzzle_from_file("input-easy.txt");
        let power_consumption = compute_power_consumption(&puzzle);
        assert_eq!(power_consumption.consumption(), 198);
    }

    #[test]
    fn test_compute_power_consumption() {
        let puzzle = read_puzzle_from_file("input.txt");
        let power_consumption = compute_power_consumption(&puzzle);
        assert_eq!(power_consumption.consumption(), 2724524);
    }
}
