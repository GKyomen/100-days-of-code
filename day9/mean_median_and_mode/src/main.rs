use std::{collections::HashMap, io};

fn main() {
    println!("Welcome to the mean, median and mode calculator in Rust!");
    println!("Please enter a sequence of whole numbers separated by spaces:");

    let mut sequence = String::new();

    io::stdin()
        .read_line(&mut sequence)
        .expect("Failed to read line");

    let split = sequence.trim().split_whitespace();

    let mut sequence: Vec<i32> = Vec::new();
    let mut counter: HashMap<i32, i32> = HashMap::new();

    for part in split {
        let num: i32 = match part.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        sequence.push(num);
        counter
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    println!(
        "The accepted inputs are in this sequence (non-numeric chars were ignored): {:?}",
        sequence
    );

    if sequence.is_empty() {
        println!("It is not possible to calculate the mean, median and mode for empty sequences.");
        return;
    }

    sequence.sort();
    let len = sequence.len();

    let mut sum = 0;
    for num in &sequence {
        sum += num;
    }

    println!("The mean is {}", sum as f32 / len as f32);

    let mid = len / 2;
    let median = if len % 2 == 0 {
        (sequence[mid - 1] + sequence[mid]) as f32 / 2.0
    } else {
        sequence[mid] as f32
    };

    println!("The median is {}", median);

    if let Some(mode) = counter.iter().max_by_key(|t| t.1).map(|(k, _v)| k) {
        println!("The mode is {}", mode);
    };
}
