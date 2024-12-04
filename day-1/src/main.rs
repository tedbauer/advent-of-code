use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::io::Lines;
use std::io::StdinLock;

fn solution2(input: &[String]) -> Result<String> {
    let (left, right) = build_left_right(input)?;

    // right_count_map[num] = how many times num appears in right.
    // left_count_map[num] = how many times num appears in left.
    let mut right_count_map: HashMap<i32, i32> = HashMap::new();
    let mut left_count_map: HashMap<i32, i32> = HashMap::new();

    for num in right {
        match right_count_map.get(&num) {
            Some(c) => right_count_map.insert(num, c + 1),
            None => right_count_map.insert(num, 1),
        };
    }

    for num in left {
        match left_count_map.get(&num) {
            Some(c) => left_count_map.insert(num, c + 1),
            None => left_count_map.insert(num, 1),
        };
    }

    let mut similarity = 0;
    for (num, count) in left_count_map.clone().into_iter() {
        let mut sim = 0;
        if let Some(right_count) = right_count_map.get(&num) {
            sim += num * right_count;
        }
        if let Some(left_count) = left_count_map.get(&num) {
            sim *= left_count;
        }
        similarity += sim;
    }

    Ok(similarity.to_string())
}

fn solution1(input: &[String]) -> Result<String> {
    let (mut left, mut right) = build_left_right(input)?;

    left.sort();
    right.sort();

    let mut result = 0;
    for (l, r) in left.into_iter().zip(right) {
        let diff = (l - r).abs();
        result += diff;
    }

    Ok(result.to_string())
}

fn build_left_right(input: &[String]) -> Result<(Vec<i32>, Vec<i32>)> {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for s in input {
        let mut split_result = s
            .split(" ")
            .filter(|s| !s.is_empty() || s.contains(char::is_whitespace));
        let left_number = split_result.next().ok_or(anyhow!("failed to parse left"))?;
        let right_number = split_result
            .next()
            .ok_or(anyhow!("failed to parse right"))?;

        left.push(left_number.parse::<i32>()?);
        right.push(right_number.parse::<i32>()?);
    }

    Ok((left, right))
}

fn main() {
    let input = io::stdin().lock().lines();
    let lines: Vec<String> = input.collect::<Result<Vec<String>, _>>().unwrap();

    println!("{}", solution1(&lines).unwrap());
    println!("{}", solution2(&lines).unwrap());
}
