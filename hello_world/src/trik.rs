use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut current_position = 1;

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<String> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        
            let directions = &nums[0];
            // println!("Direction {:?}", directions);

            for direction in directions.chars() {
                // if current_position == 1 && direction == 'A'{
                //     // println!("1->A");
                //     current_position += 1;
                // } else if current_position == 1 && direction == 'C' {
                //     // println!("1->C");
                //     current_position += 2;
                // } else if current_position == 2 && direction == 'B' {
                //     // println!("2->B");
                //     current_position += 1;
                // } else if current_position == 2 && direction == 'A' {
                //     // println!("2->A");
                //     current_position += -1;
                // } else if current_position == 3 && direction == 'B' {
                //     // println!("3->B");
                //     current_position += -1;
                // } else if current_position == 3 && direction == 'C' {
                //     // println!("3->C");
                //     current_position += -2;
                // } 
                match (current_position, direction) {         
                    (1, 'A') => current_position += 1,
                    (1, 'C') => current_position += 2,
                    (2, 'B') => current_position += 1,
                    (2, 'A') => current_position += -1,
                    (3, 'B') => current_position += -1,
                    (3, 'C') => current_position += -2,
                    (_, _) => current_position += 0,
                }
            }

            println!("{}", current_position);
            current_position = 1;
        
  }
}
