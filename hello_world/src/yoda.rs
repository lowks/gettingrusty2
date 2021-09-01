use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut all_strings: Vec<Vec<String>> = Vec::new();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<String> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        all_strings.push(nums);

        if all_strings.len() == 2 {
            let mut input_1 = all_strings[0][0].clone();
            let mut input_2 = all_strings[1][0].clone();
            
            if input_1.len() > input_2.len() {
                let diff = input_1.len() - &input_2.len();
                if diff == 1 {
                    input_2 = "0".to_owned() + &input_2.clone();
                } else {
                      for _time in 0..input_1.len() - input_2.len() {
                        input_2 = "0".to_owned() + &input_2.clone();
                      }
                }
             } else if input_2.len() > input_1.len() {
                let diff = input_2.len() - &input_1.len();
                if diff == 1 {
                    input_1 = "0".to_owned() + &input_1.clone();
                } else {
                      for _time in 0..input_2.len() - input_1.len() {
                        input_1 = "0".to_owned() + &input_1.clone();
                      }
                }
             }
            
            let mut final_output_1:Vec<String> = Vec::new();
            let mut final_output_2:Vec<String> = Vec::new();
            
            let input_1_vec = input_1
                              .chars()
                              .map(|d| d.to_string().parse::<i32>().unwrap())
                              .collect::<Vec<_>>().to_vec();
          
            let input_2_vec = input_2
                              .chars()
                              .map(|d| d.to_string().parse::<i32>().unwrap())
                              .collect::<Vec<_>>().to_vec();
          
            for iterator in 0..input_1_vec.len() {
                let x = input_1_vec[iterator];
                let y = input_2_vec[iterator];
                
                if x > y {
                        final_output_1.push(x.to_string());
                } else if y > x {
                    final_output_2.push(y.to_string());
                } else if x == y {
                    final_output_1.push(x.to_string());
                    final_output_2.push(y.to_string());
                } 
                
            }
            for item in [final_output_1, final_output_2].iter() {
                if item.len() == 0 {
                    println!("YODA");
                } else if item.iter().filter(|&n| *n == "0".to_string()).count() == item.len() {
                    println!("0");
                } else {
                    println!("{}",item.join(""));
                }
            }
        }

    }
}
