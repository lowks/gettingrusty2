use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut times = 0;
    let mut all_lines: Vec<Vec<f32>> = Vec::new();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<f32> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        
            // println!("Direction {:?}", directions);
            all_lines.push(nums);

            times = all_lines[0][0] as usize;
            // println!("All Lines: {:?} {}", all_lines, times);

            // if all_lines.len() == times + 1 {
               
            // }
            // current_position = 1;

            if all_lines.len() == times + 1 {
                for input in 1..times + 1 {
                    let bottles = &all_lines[input][0]/400.0;
                    println!("{}", bottles.ceil());
                }
            }
        
  }
}
