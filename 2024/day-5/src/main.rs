use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

fn solve_part_one(input: &[String]) -> Result<String> {
    let correct_updates = compute_correct_updates(input)?;
    Ok(sum_middle_pages(correct_updates).to_string())
}

fn solve_part_two(input: &[String]) -> Result<String> {
    let fixed_updates = compute_fixed_updates(input)?;
    Ok(sum_middle_pages(fixed_updates).to_string())
}

fn compute_fixed_updates(input: &[String]) -> Result<HashSet<Vec<usize>>> {
    let must_be_before: HashMap<usize, HashSet<usize>> = compute_must_be_before(input)?;
    let mut fixed_updates: HashSet<Vec<usize>> = HashSet::new();
    for line in input {
        // We just want to process the updates, so skip the first section.
        if line.contains("|") {
            continue;
        } else if line.is_empty() {
            continue;
        } else {
            let update: Result<Vec<usize>, _> =
                line.split(",").map(|n| n.parse::<usize>()).collect();
            let mut update: Vec<usize> = update?;

            let mut fixed = false;
            let mut applied_fix = false;
            while !fixed {
                let mut discovered_invalid = false;
                for (i, page) in update.clone().into_iter().enumerate() {
                    if discovered_invalid {
                        break;
                    }

                    if i == update.len() - 1 {
                        fixed = true;

                        if applied_fix {
                            fixed_updates.insert(update.clone());
                            applied_fix = false;
                        }
                        break;
                    }

                    for (other_i, other_page) in update[i + 1..].into_iter().enumerate() {
                        if must_be_before
                            .get(&other_page)
                            .map(|before| before.contains(&page))
                            .unwrap_or(false)
                        {
                            discovered_invalid = true;
                            let other_index = other_i + i + 1;
                            update.swap(i, other_index);
                            applied_fix = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    Ok(fixed_updates)
}

fn sum_middle_pages(updates: HashSet<Vec<usize>>) -> usize {
    updates
        .into_iter()
        .map(|update| update.get(update.len() / 2).unwrap_or(&0).clone())
        .sum()
}

fn compute_must_be_before(input: &[String]) -> Result<HashMap<usize, HashSet<usize>>> {
    let mut must_be_before: HashMap<usize, HashSet<usize>> = HashMap::new();
    for line in input {
        // Populate must_be_before.
        if line.contains("|") {
            let mut split = line.split("|");
            let (left, right) = (
                split
                    .next()
                    .ok_or(anyhow!("failed to parse left"))?
                    .parse::<usize>()?,
                split
                    .next()
                    .ok_or(anyhow!("failed to parse right"))?
                    .parse::<usize>()?,
            );

            must_be_before
                .entry(left)
                .or_insert(HashSet::new())
                .insert(right);
        }
        // Empty line, so stop processing.
        else if line.is_empty() {
            break;
        }
    }
    Ok(must_be_before)
}

fn compute_correct_updates(input: &[String]) -> Result<HashSet<Vec<usize>>> {
    let must_be_before = compute_must_be_before(input)?;
    let mut correct_updates: HashSet<Vec<usize>> = HashSet::new();

    for line in input {
        // We just want to process the updates, so skip the first section.
        if line.contains("|") {
            continue;
        } else if line.is_empty() {
            continue;
        } else {
            let update: Result<Vec<usize>, _> =
                line.split(",").map(|n| n.parse::<usize>()).collect();
            let update: Vec<usize> = update?;

            let mut discovered_invalid = false;
            for (i, page) in update.clone().into_iter().enumerate() {
                if discovered_invalid {
                    break;
                }

                if i == update.len() - 1 {
                    correct_updates.insert(update);
                    break;
                }

                for other_page in &update[i + 1..] {
                    if must_be_before
                        .get(&other_page)
                        .map(|before| before.contains(&page))
                        .unwrap_or(false)
                    {
                        discovered_invalid = true;
                        break;
                    }
                }
            }
        }
    }

    Ok(correct_updates)
}

fn main() -> Result<()> {
    let input = io::stdin().lock().lines();
    let lines: Vec<String> = input
        .collect::<Result<Vec<String>, _>>()
        .map_err(|err| anyhow!("failed to parse lines: {}", err))?;

    println!("{}", solve_part_one(&lines)?);
    println!("{}", solve_part_two(&lines)?);

    Ok(())
}
