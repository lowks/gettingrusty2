use std::io::{self, BufRead};
// use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let mut all_lines: Vec<i32> = Vec::new();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i32> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        all_lines.push(nums[0]);
        if all_lines.len() == 10 {
            let mut count_vec: Vec<i32> = Vec::new();
            for number in &all_lines {
                count_vec.push(*number % 42);
            }
            // count_vec = count_vec.into_iter().sorted().dedup().collect();
            count_vec = {
                count_vec.sort();
                count_vec.dedup();
                count_vec
            };
            println!("{}", count_vec.len());
        }
    }
}