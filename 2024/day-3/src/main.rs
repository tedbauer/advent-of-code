use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

fn solve_part_one(input: &[String]) -> Result<String> {
    let correct_updates = compute_correct_updates(input)?;

    let mut page_sum = 0;
    for update in correct_updates {
        if let Some(middle_page_num) = update.get(update.len() / 2) {
            page_sum += middle_page_num
        }
    }

    Ok(page_sum.to_string())
}

fn compute_correct_updates(input: &[String]) -> Result<HashSet<Vec<usize>>> {
    let mut must_be_before: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut correct_updates: HashSet<Vec<usize>> = HashSet::new();

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
        // Empty line.
        else if line.is_empty() {
            continue;
        }
        // Process an entry.
        else {
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

fn main() {
    let input = io::stdin().lock().lines();
    let lines: Vec<String> = input.collect::<Result<Vec<String>, _>>().unwrap();

    println!("{}", solve_part_one(&lines).unwrap());
}
