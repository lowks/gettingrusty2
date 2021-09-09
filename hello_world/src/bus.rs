use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut all_lines: Vec<Vec<i32>> = Vec::new();
    let mut times = 0;


    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i32> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        all_lines.push(nums);

        if all_lines.len() == 1 {
            times = all_lines[0][0] as usize;
        }

        if all_lines.len() == times + 1  {
            // println!("{:?}", all_lines);
            for kay in 1..all_lines.len() {
                let number_of_stop = all_lines[kay][0];
                // println!("number_of_stop {}", number_of_stop);
                let mut original_total = 0;

                if number_of_stop == 1 {
                    original_total = 0;
                    println!("{}", 1);
                    continue;
                } else if number_of_stop == 0 {
                    original_total = 0;
                    println!("{}", 0);
                    continue;
                } else if number_of_stop > 1 {
                    original_total = 0;
                    // for stop in 0..number_of_stop {
                    //     // println!("kay {}", kay);
                    //     original_total += i32::pow(2, stop.to_string().parse::<u32>().unwrap());
                    // }
                    let mut inner_number_of_stop = number_of_stop;
                    while inner_number_of_stop > 0 {
                        original_total += i32::pow(2, inner_number_of_stop.to_string().parse::<u32>().unwrap() - 1);
                        // println!("original_total {}", original_total);
                        inner_number_of_stop -= 1;
                    }
                    println!("{}", original_total);
                    continue;
                }
            }
        }
    }
}
