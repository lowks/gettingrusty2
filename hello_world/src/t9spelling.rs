use std::io::{self, BufRead};
use std::iter;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines = 0;
    let mut nums_vec: Vec<String> = Vec::new();

    let mut pm: HashMap<char, Vec<_>> = HashMap::new();
    pm.insert('a', vec![2,1]);
    pm.insert('b', vec![2,2]);
    pm.insert('c', vec![2,3]);

    pm.insert('d', vec![3,1]);
    pm.insert('e', vec![3,2]);
    pm.insert('f', vec![3,3]);

    pm.insert('g', vec![4,1]);
    pm.insert('h', vec![4,2]);
    pm.insert('i', vec![4,3]);

    pm.insert('j', vec![5,1]);
    pm.insert('k', vec![5,2]);
    pm.insert('l', vec![5,3]);

    pm.insert('m', vec![6,1]);
    pm.insert('n', vec![6,2]);
    pm.insert('o', vec![6,3]);

    pm.insert('p', vec![7,1]);
    pm.insert('q', vec![7,2]);
    pm.insert('r', vec![7,3]);
    pm.insert('s', vec![7,4]);

    pm.insert('t', vec![8,1]);
    pm.insert('u', vec![8,2]);
    pm.insert('v', vec![8,3]);

    pm.insert('w', vec![9,1]);
    pm.insert('x', vec![9,2]);
    pm.insert('y', vec![9,3]);
    pm.insert('z', vec![9,4]);

    pm.insert(' ', vec![0,1]);

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: String = line;

            nums_vec.push(nums);

            if nums_vec.len() == 1 {
                lines = nums_vec[0].parse::<i32>().unwrap() as usize;
            }

            if nums_vec.len() == lines + 1 {

                for iterator in 1..nums_vec.len() {
                    let mut output_string = String::new();
                    // println!("iterator {}", iterator);
                    // let get_code = pm.get(&char).unwrap();
                    // println!("{} {:?}", iterator, nums_vec[iterator]);
                    let input_line = &nums_vec[iterator];
                    for char in input_line.chars() {
                        let get_code = pm.get(&char).unwrap();
                        if output_string.ends_with(&get_code[0].to_string()) {
                            // println!("hoho {} output_string: {}", &get_code[0].to_string(), output_string);
                            output_string += &" ".to_string();
                            output_string += &iter::repeat(get_code[0].to_string()).take(get_code[1]).collect::<String>();
                        } else { 
                            // println!("{} output_string: {}", &get_code[0].to_string(), output_string);
                            output_string += &iter::repeat(get_code[0].to_string()).take(get_code[1]).collect::<String>();
                        }
                    }
                    println!("Case #{}: {}",iterator, output_string);
                }
                // println!("{:?}", nums_vec);
            }
            // if nums_vec.len() == lines + 1 {
               
            // }

  }
}
