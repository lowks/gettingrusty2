use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i32> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let sequence = nums;
        recurse(sequence);
        
    }
  }

fn recurse(real_numbers:Vec<i32>) {
    let real_numbers_clone = &mut real_numbers.clone();
    for index in 0..real_numbers_clone.len() - 1 {
        let number1 = real_numbers_clone[index];
        let number2 = real_numbers_clone[index+1];
        if number1 > number2 {
            real_numbers_clone.swap(index, index+1);
            println!("{} {} {} {} {}", real_numbers_clone[0], real_numbers_clone[1], real_numbers_clone[2], real_numbers_clone[3], real_numbers_clone[4]);
        }
    }

    return if real_numbers_clone == &vec![1,2,3,4,5] {
        ();
    } else if real_numbers_clone != &vec![1,2,3,4,5] {
        recurse(real_numbers_clone.to_vec());
    }
}
