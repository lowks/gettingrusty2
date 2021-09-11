use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut all_lines: Vec<Vec<i32>> = Vec::new();
    // let mut a_agg = 0;
    // let mut a_calm = 0;
    // let mut b_agg = 0;
    // let mut b_calm = 0;
    

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i32> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        all_lines.push(nums);

	if all_lines.len() == 2 {
    //    a_agg = all_lines[0][0];
    //    a_calm = all_lines[0][1];
    //    b_agg = all_lines[0][2];
    //    b_calm = all_lines[0][3];
    // let settings = vec![a_agg, a_calm, b_agg, b_calm];
    let [a_agg, a_calm, b_agg, b_calm] = [ all_lines[0][0], all_lines[0][1], all_lines[0][2], all_lines[0][3] ];
    // let settings = all_lines[0];
    // let all_lines[0] = [a_agg, a_calm, b_agg, b_calm];
       
       for input_number in &all_lines[1] {
            let mut bitten = 0;
            // println!("input number {}", input_number);
            if ( input_number % (a_agg + a_calm) ) > a_agg || ( input_number % (a_agg + a_calm) ) == 0 {
                // println!("I am calm a");
                ();
                } else {
                    bitten += 1;
            }
                
            if ( input_number % (b_agg + b_calm) ) > b_agg || ( input_number % (b_agg + b_calm) ) == 0 {
                // println!("I am calm b");
                ();
            } else {
                bitten += 1;
            }
        
            if bitten == 2 {
                    println!("both");
            } else if bitten == 0 {
                    println!("none");
            } else if bitten == 1 {
                    println!("one");
            }
       }
    }
  }
}