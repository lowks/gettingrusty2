use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut all_lines: Vec<Vec<i32>> = Vec::new();
    let mut element: Vec<i32> = Vec::new();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i32> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        
        all_lines.push(nums);

        if all_lines.len() == 2 {
            let input_total = all_lines[0][1];
            let mut total_task = all_lines[0][0];
            let vec = &all_lines[1];
            let mut total = 0;
            
            for number in vec {
                total = element.iter().sum();

                if total_task >= 0 {
                    match total < input_total {
                        true => {
                            element.push(*number);
                            total_task -= 1;
                        },
                        false => {
                            break;
                        }, 
                    }
                }
            }
            match total > input_total {
                true => println!("{}", element.len() - 1),
                false => println!("{}", element.len()),
            }
        }
    }
}