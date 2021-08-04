use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut all_lines: Vec<_> = Vec::new();
    let mut result:f64 = 0.0;
    let mut times = 0;
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<f64> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

    if nums.len() == 1 {
        times = nums[0] as i64;
    } else if nums.len() == 2 {
        all_lines.push(nums)
    }

    // let all_lines_length:i64 = all_lines.len() as i64;
    // println!("all_lines_length ====> {}", all_lines_length);
    // println!("times ====> {}", times);


    if all_lines.len() as i64  == times  {
        // println!("all_lines_length ====> {}", all_lines_length);
        for line in &all_lines {
          // println!("{} X {}", line[0], line[1]);
          result += line[0] as f64 * line[1] as f64;
        }
       println!("{:.3}", result);
    }
  }
}
