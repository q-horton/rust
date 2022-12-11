use std::io;
use std::collections::HashMap;

fn sort(mut ints: Vec<i32>) -> Vec<i32> {
    let mut ordered: Vec<i32> = Vec::new();
    let length = ints.len();
    for _i in 0..length {
        let mut index = 0;
        for i in 0..ints.len() {
            if ints[i] < ints[index] {
                index = i;
            }
        }
        ordered.push(ints.remove(index));
    }
    ordered
}

fn mean(ints: &Vec<i32>) -> f64 {
    let mut sum: f64 = 0.0;
    let n = ints.len() as f64;
    for i in ints {
        sum += *i as f64;
    }
    sum / n
}

fn median(ordered: &Vec<i32>) -> f64 {
    let mid = ordered.len() / 2;
    if ordered.len() % 2 == 0 {
        return ((ordered[mid - 1] + ordered[mid]) as f64) / 2f64;
    } else {
        return ordered[mid] as f64;
    }
}

fn mode(ints: &Vec<i32>) -> Vec<i32> {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for i in ints {
        let count = counts.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut max_count = 0;
    for (_key, value) in &counts {
        if *value > max_count {
            max_count = *value;
        }
    }
    let mut modes: Vec<i32> = Vec::new();
    for (key, value) in &counts {
        if *value == max_count {
            modes.push(*key);
        }
    }
    modes
}

fn int_vec_to_str(nums: Vec<i32>) -> String {
    let mut result = String::new();
    for i in nums {
        if result.len() == 0 {
            result.push_str(&i.to_string());
        } else {
            result.push_str(", ");
            result.push_str(&i.to_string());
        }
    }
    result
}

pub fn main() {
    println!("Mean, Median and Mode");
    println!("Please enter an array of integers followed by the word 'end'.");

    let mut ints: Vec<i32> = Vec::new();

    loop {
        let mut current: String = String::new();
        io::stdin()
            .read_line(&mut current)
            .expect("Failed to read line.");
        
        if current.trim() == "end" {
            break;
        }

        let current: i32 = match current.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{} was not a valid entry, it will be ignored", current.trim());
                continue;
            }
        };

        ints.push(current);
    }
    if ints.len() == 0 {
        ints.push(0);
    }

    let ordered = sort(ints);
    let mean = mean(&ordered);
    let median = median(&ordered);
    let modes = mode(&ordered);
    let mode_str = int_vec_to_str(sort(modes));
    println!("The mean is {mean}, the median is {median}, the modes are {mode_str}");
}