use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut all_lines: Vec<String> = Vec::new();
    let mut max_correct_answer = 0;
    let mut count_your_correct = 0;
    let mut different_answer:i32= 0;
    let mut wrong_answer:i32 = 0;
    let mut final_score:i32 = 0;

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: String = line;
            // .map(|num| num.parse().unwrap())
            // .collect();
        
        all_lines.push(nums);
        // println!("{:?}", all_lines);
        // let name = &nums;
        if all_lines.len() == 3 {
            max_correct_answer = all_lines[0].parse::<i32>().unwrap();
            let mut friend_answer: Vec<char> = all_lines[2].chars().collect();
            friend_answer.sort();
            let mut your_answer: Vec<char> = all_lines[1].chars().collect();
            your_answer.sort();

            // println!("Friend {:?} Your {:?}", friend_answer, your_answer);

            for iterator in 0..friend_answer.len() {
                if friend_answer[iterator] == your_answer[iterator] {
                    count_your_correct += 1;
                }
            }

            different_answer = your_answer.len() as i32 - count_your_correct;
            
            if count_your_correct >= max_correct_answer {
                wrong_answer = count_your_correct as i32 - max_correct_answer;
                final_score = count_your_correct + different_answer - wrong_answer;
            } else if max_correct_answer > count_your_correct {
                final_score = count_your_correct;
            }

            println!("{}", final_score);

        }
    }
}
