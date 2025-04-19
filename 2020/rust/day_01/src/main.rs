use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_nums(filename: &str) -> io::Result<Vec<i32>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut numbers: Vec<i32> = Vec::new();

    for line_res in reader.lines() {
        let line = line_res?;
        let number: i32 = line.trim().parse().map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid number: {}", e))
        })?;
        numbers.push(number);
    }

    Ok(numbers)
}

fn find_two(target: i32, collection: &HashSet<i32>) -> Option<(i32, i32)> {
    for &item in collection {
        let looking_for = target - item;
        if collection.contains(&looking_for) {
            return Some((item, looking_for));
        }
    }

    None
}

fn find_three(target: i32, collection: &HashSet<i32>) -> Option<(i32, i32, i32)> {
    for &item in collection {
        let looking_for = target - item;
        if let Some((a, b)) = find_two(looking_for, collection) {
            return Some((item, a, b));

        }
    }
    None
}

fn main() -> io::Result<()> {
    println!("Solving Day 01 in Rust");
    let file = "../../data/day_01_p1.txt";
    let numbers = read_nums(file)?;
    let numbers_set: HashSet<i32> = numbers.iter().cloned().collect();
    println!("number list size: {:?}", numbers.len());
    println!("number set size: {:?}", numbers_set.len());

    match find_two(2020, &numbers_set) {
        Some((a, b)) => println!("Found: {}, {}, {}, {}", a, b, a + b, a * b),
        None => println!("No matching pair found"),
    }

    match find_three(2020, &numbers_set) {
        Some((a, b, c)) => println!("Found: {}, {}, {}, {}, {}", a, b, c, a + b + c, a * b * c),
        None => println!("No matching pair found"),
    }

    Ok(())
}
