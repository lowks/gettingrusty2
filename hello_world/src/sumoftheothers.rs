use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    // let mut all_strings: Vec<Vec<String>> = Vec::new();
    // let mut blimps: Vec<String> = Vec::new();
    

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i32> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let mut vec = nums;
        let mut other_vec = vec.clone();
        let mut vec_sum:i32 = other_vec.iter().sum();


        for number in vec {
            // if number > 0 {
            //     if vec_sum - number == number {
            //         println!("{}", number);
            //         break;
            //     }
            // } else if number < 0 {
            //     if vec_sum + number == number {
            //         println!("{}", number);
            //         break;
            //     }
            // }

            match number > 0 {
                true => {
                    if vec_sum - number == number {
                        println!("{}", number);
                        break;
                    } 
                },

                false => {
                    if vec_sum + number == number {
                        println!("{}", number);
                        break;
                    }
                },
            }

            // let final_output   = match number < 0 {
            //                 false if vec_sum - number == number => number,
            //                 true if vec_sum + number == number => number,
            //                 _ => 0,
            // };
            // println!("{:?}", final_output);
            // break;
        }
    
    //     for iterator in 0..vec.len() {
    //         let number = vec[iterator] as i32;
    //         let filtered = other_vec.iter().filter(|&n| *n != number).collect::<Vec<_>>();
    //         let count = other_vec.iter().filter(|&n| *n == number).count();
    //         let i32_count = count.to_string().parse::<i32>().unwrap();
    //         let mut filtered_total:i32 = 0;
    //         // println!("filtered {:?}", filtered);
    //         for number in filtered {
    //             // println!("number {}", number);
    //             filtered_total += number;
    //         }
    //         // println!("filtered_total {}", filtered_total);
            
    //         if count > 1 {
    //             // if number > 0 {
    //             filtered_total += (i32_count - 1) * (number);
    //             // } else if number < 0 {
    //                 // filtered_total  += i32_count * (number + 1);
    //             // }
    //         }
    //         // for number in 
    //         // println!("Number {}, filtered_total {} i32_count {}",number,filtered_total,i32_count);
    //         if number == filtered_total {
    //             println!("{}", number);
    //             break;
    //         }
    //         // let sum: i32 = filtered.clone().iter().sum();
    //         // if &filtered.iter().sum() == number {
    //         //     println!("{}", number);
    //         // }
    // }

 }
}
