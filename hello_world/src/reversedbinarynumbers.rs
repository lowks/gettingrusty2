use std::io::{self, BufRead};
// use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
	// let mut all_lines: Vec<_> = Vec::new();
	// let mut javelins_to_expect = 0;
	// let mut total_length = 0;
	// let total_javelines 
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i32> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
		let input_number = nums[0];
		let binary_val = format!("{:b}", input_number);
		let reversed_val = binary_val.chars().rev().collect::<String>();
		let to_show = isize::from_str_radix(&reversed_val, 2).unwrap();
		println!("reversed {}", to_show);
  }
}
