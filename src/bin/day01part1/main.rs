use std::fs;
use std::process;

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

    if left.len() != right.len() {
        eprintln!("Left and right vector sizes do not match");
        process::exit(1);
    }

    left.sort();
    right.sort();

    let mut sum: i64 = 0;
    for (i, item) in left.iter().enumerate() {
        let difference = (item - right[i]).abs();
        sum += difference;
        println!("{} {} {} {}", i, item, right[i], sum);
    }

    println!("Sum of differences: {}", sum);
}
