use std::fs;

const MIN_DIFFERENCE: i32 = 1;
const MAX_DIFFERENCE: i32 = 3;

fn main() {
    let contents: Vec<String> = fs::read_to_string("inputs/day02.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut count = 0;

    for line in contents {
        if is_safe(&line) {
            count += 1;
        }
    }

    println!("There were {count} safe reports");
}

fn is_safe(line: &String) -> bool {
    let mut is_ascending: Option<bool> = None;
    let parts: Vec<&str> = line.split(" ").collect();
    let length = parts.len();

    if length < 2 {
        panic!("Not enough parts provided!");
    }

    for i in 1..length {
        let left: i32 = parts[i - 1].parse().expect("left not a number");
        let right: i32 = parts[i].parse().expect("right not a number");

        if left < right { // ascending
            match is_ascending {
                None => is_ascending = Some(true),
                Some(false) => return false,
                _ => ()
            }
        } else if left > right { // descending
            match is_ascending {
                None => is_ascending = Some(false),
                Some(true) => return false,
                _ => ()
            }
        }

        let diff = (left - right).abs();
        if diff < MIN_DIFFERENCE || diff > MAX_DIFFERENCE {
            return false
        }
    }

    return true
}