use std::io::{self, BufRead};
use std::collections::HashMap;
// use itertools::Itertools;

fn main() {
    let stdin = io::stdin();
    let mut all_lines: Vec<Vec<String>> = Vec::new();
    let mut entry_exit: HashMap<String, String> = HashMap::new();
    let mut times = 0;

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<String> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        
        all_lines.push(nums);

        if all_lines.len() == 1 {
            times = all_lines[0][0].parse::<i32>().unwrap() as usize;
        }

        if all_lines.len() == times + 1 {
            for iterator in 1..all_lines.len() {
                let action = &all_lines[iterator][0];
                let name = &all_lines[iterator][1];

                if action == "entry" {
                    match entry_exit.get(&name.clone()) {
                            Some(x) => {
                                if x == "exit" {
                                    println!("{} entered", name);
                                } else if x == "entry" {
                                    println!("{} entered (ANOMALY)", name);
                                }
                            },
                            None => {
                                println!("{} entered", name);
                            },
                    }
                    entry_exit.insert(name.to_string(), action.to_string());
                } else if action == "exit" {
                        match entry_exit.get(&name.clone()) {
                            Some(x) => {
                                if x == "entry" {
                                    println!("{} exited", name);
                                } else if x == "exit" {
                                    println!("{} exited (ANOMALY)", name);
                                }
                            },
                            None => {
                                println!("{} exited (ANOMALY)", name);
                            },
                        }
                        entry_exit.insert(name.to_string(), action.to_string());
                }
            }
        }
    }
}