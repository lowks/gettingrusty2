use std::io::{self, BufRead};
use std::collections::HashMap;


fn main() {
    let stdin = io::stdin();
    let mut all_strings: Vec<String> = Vec::new();
    let mut m: HashMap<String, i32> = HashMap::new();
    let mut final_result: Vec<String> = Vec::new();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let mut nums: String = line;
        
        all_strings.push(nums);

        // for x in &all_strings {
        //     if x != "***" {
        //         *m.entry(x.to_string()).or_default() += 1;
        //     } else {
        //         // let max = m.clone().values().max();
        //         for (k, v) in m {
        //             if v == *m.clone().values().max().unwrap() {
        //                 final_result.push(k);
        //             }
        //         }
        //     }
        //     if final_result.len() > 1 {
        //         println!("Runoff!");
        //     } else {
        //         println!("{}", final_result[0]);
        //     }
        // }
        // println!("all strings {:?}", &all_strings);
        if all_strings.contains(&"***".to_string()) {
            for x in &all_strings {
                if x == "***" {

                    let max_value = m.clone();
                    // println!("m {:?}", m);
                    
                    // for (k, v) in &max_value {
                    //     if *v == *max_value.values().max().unwrap() {
                    //         final_result.push(k.to_string());
                    //     }
                    // }
                    let highest_value = *max_value.values().max().unwrap();
                    // println!("Highest value {}", highest_value);
                    if max_value.values().filter(|&n| *n == highest_value).count() == 1 {
                        // println!("In here");
                        for (k, v) in &max_value {
                                if *v == highest_value {
                                        final_result.push(k.to_string());
                                }
                        }
                        println!("{}", final_result[0]);
                    } else {
                        println!("Runoff!");
                        break;
                    }
                } else {
                    *m.entry(x.to_string()).or_default() += 1;
                }
            }
        }      
        // let max_value = m.clone();
        // println!("m {:?}", m);
        
        // for (k, v) in &max_value {
        //     if *v == *max_value.values().max().unwrap() {
        //         final_result.push(k.to_string());
        //     }
        // }

        // println!("{:?}", final_result);
        
        // if final_result.len() > 1 {
        //     println!("final result {:?}", final_result);
        //     println!("Runoff!");
        // } else {
        //     println!("{}", final_result[0]);
        // }

        // let max = m.clone().into_iter().max_by_key(|(_, v)| *v).map(|(k, _)| k);
        // let max = m.values().max();
        // println!("m ===> {:?}", find_keys_for_value(&m, *max.unwrap()));
        // println!("{}", final_result[0]);
        
        // if final_result.len() > 1 {
        //     println!("Runoff!")
        // } else if final_result.len() == 1 {
        //     println!("{}", final_result[0]);
        // }
  }
}