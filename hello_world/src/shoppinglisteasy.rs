use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut number_of_list = 0;
    let mut len_of_list = 0;
    let mut duplicate = 0;
    let mut shopping_lists: Vec<Vec<String>> = Vec::new();
    let mut shopping_items: Vec<String> = Vec::new();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<String> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        
        let mut lists_map: HashMap<String, usize> = HashMap::new();
        
            shopping_lists.push(nums);
            if shopping_lists.len() == 1 {
                number_of_list = shopping_lists[0][0].parse::<usize>().unwrap();
                len_of_list = shopping_lists[0][1].parse::<usize>().unwrap();
                // println!("{} {}", number_of_list, len_of_list);
            }

            if shopping_lists.len() == number_of_list + 1 {
                for iterator in 1..number_of_list + 1 {
                    for item in shopping_lists[iterator].clone() {
                        *lists_map.entry(item).or_insert(0) += &1;
                        // println!("item {}", item);
                    }
                }
                for (key, value) in &lists_map {
                    if *value == number_of_list {
                        duplicate += 1;
                        shopping_items.push(key.to_string());
                    }
                }
                println!("{}", duplicate);
                shopping_items.sort();
                for line in &shopping_items {
                    println!("{}", line);
                }
            }
  }
}
