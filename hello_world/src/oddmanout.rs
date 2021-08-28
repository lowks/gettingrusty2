use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut all_lines: Vec<Vec<usize>> = Vec::new();
    let mut number_of_guests = 0;
    let mut case_number = 1;
    let mut tickets: &Vec<usize> = &Vec::new();
    let mut odd_man_out = 0;
    let mut iterator_elements = 1;

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<usize> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        
        all_lines.push(nums);

        if all_lines.len() > 1 {
            number_of_guests = all_lines[0][0];
        }

        if all_lines.len() == number_of_guests*2 + 1 {
            for _iterator in 1..((all_lines.len() - 1)/2) + 1 {
                let all_lines_clone = all_lines.clone();
                number_of_guests = all_lines_clone[iterator_elements][0];
                tickets = &all_lines_clone[iterator_elements + 1];
                while number_of_guests > 0 {
                    for ticket in tickets {
                        if tickets.iter().filter(|&n| *n == *ticket).count() == 1 {
                            odd_man_out = *ticket;
                        }
                        number_of_guests -= 1;
                    }
                }
                println!("Case #{}: {}", case_number, odd_man_out);
                case_number += 1;
                iterator_elements += 2;
            }
        }

        
    }
}