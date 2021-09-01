use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut all_lines: Vec<Vec<usize>> = Vec::new();
    

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<usize> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        all_lines.push(nums);

        if all_lines.len() == 2 {
            let mut input_vector: Vec<usize> = all_lines[1].clone();
            let mut output_string:Vec<String> = Vec::new();
            output_string.push("1".to_string());
            let mut input_vec_clone = input_vector.clone();
            input_vec_clone.sort();
            
            for space in &input_vec_clone {
                output_string.push((input_vector.iter().position(|&r| r == *space).unwrap() + 2).to_string());
            }
            println!("{}", output_string.join(" "));
        }
        
    }
}