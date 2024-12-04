use anyhow::Result;
use std::io;
use std::io::BufRead;

fn solve_part_two(lines: &[String]) -> Result<u32> {
    
}

fn solve_part_one(lines: &[String]) -> Result<u32> {
    let mut safe_count = 0;

    for line in lines {
        let numbers: Result<Vec<i32>, _> = line.split(' ').map(|n| n.parse::<i32>()).collect();

        let mut differences = Vec::new();
        for pair in numbers?.windows(2) {
            differences.push(pair[0] - pair[1]);
        }

        let all_same_direction =
            differences.iter().all(|d| *d > 0) || differences.iter().all(|d| *d < 0);
        let all_in_range = differences.iter().all(|d| d.abs() >= 1 && d.abs() <= 3);

        if all_same_direction && all_in_range {
            safe_count += 1
        }
    }

    Ok(safe_count)
}

fn main() {
    let input = io::stdin().lock().lines();
    let lines: Vec<String> = input.collect::<Result<Vec<String>, _>>().unwrap();

    println!("{}", solve_part_one(&lines).unwrap())
}
