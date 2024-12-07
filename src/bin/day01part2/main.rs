use std::fs;
use std::collections::HashMap;

fn main() {
    // Read lines into vector
    let contents: Vec<String> = fs::read_to_string("inputs/day01.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    // Split lines and store left/right side in their respective vectors
    for line in contents {
        let parts: Vec<&str> = line.split("   ").collect();

        left.push(parts[0].parse::<i64>().expect("not a valid number on left"));
        right.push(parts[1].parse::<i64>().expect("not a valid number on right"));
    }

    let mut counts = HashMap::<i64, i64>::new();

    // Count the number of occurrences in right vector
    for num in right {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mut similarity: i64 = 0;

    for num in left {
        let count = counts.get(&num).unwrap_or(&0);
        let score = num * count;
        similarity += score;
    }

    println!("Total similarity score: {}", similarity);
}
