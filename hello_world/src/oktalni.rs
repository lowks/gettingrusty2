use std::io::{self, BufRead};
use std::io::Write;


fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let mut nums: Vec<String> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

		let input_string = &mut nums[0].clone();
		while input_string.len() % 3 != 0 {
            input_string.insert_str(0, "0");
        }
    
        let mut iterator = 0;

        while iterator + 3 <= input_string.len() {
            print!("{}", isize::from_str_radix(&input_string[iterator..iterator+3], 2).unwrap());
            io::stdout().flush().unwrap();
            iterator += 3;
        }
        println!("");

  }
}
