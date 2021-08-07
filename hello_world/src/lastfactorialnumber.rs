use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut all_lines: Vec<_> = Vec::new();
    let mut how_many_lines = 0;

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<u64> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        all_lines.push(nums);

	if all_lines.len() == 1 {
       how_many_lines = all_lines[0][0] + 1;
    }
    
    if all_lines.len() == how_many_lines as usize {
        let lines = all_lines.drain(1..);
        for line in lines {
            let number = line[0];
            match number {
                1 => println!("{}", "1".to_string()),
                2 => println!("{}", "2".to_string()),
                3 => println!("{}", "6".to_string()),
                4 => println!("{}", "4".to_string()),
                _ => println!("{}", "0".to_string()),
            }
        }
     }
  }
}
