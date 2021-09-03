use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut all_strings: Vec<Vec<String>> = Vec::new();
    let mut max_people = 0;
    let mut test_cases = 0;

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<String> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        all_strings.push(nums);

        if all_strings.len() == 1 {
           max_people = all_strings[0][0].parse::<i32>().unwrap();
           test_cases = all_strings[0][1].parse::<usize>().unwrap();
        //    println!("max_people {} test_cases {}", max_people, test_cases);
        }

        if all_strings.len() == test_cases + 1 {
            // println!("{:?}", all_strings);
            let mut action = "";
            let mut current_people = 0;
            let mut denied = 0;
            // let number = 0;
            for iterator in 1..all_strings.len() {
                action = &all_strings[iterator][0];
                let number = all_strings[iterator][1].parse::<i32>().unwrap();

                if action == "enter" && current_people + number <= max_people {
                    current_people += number;
                } else if action == "enter" && current_people + number > max_people {
                    denied += 1
                } else if action == "leave" {
                    current_people -= number;
                }
            }
            println!("{}", denied);
        }
    }
}
