use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut all_lines: Vec<String> = Vec::new();
    let mut max_correct_answer = 0;
    let mut count_same = 0;
    let mut count_different = 0;

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: String = line;
        all_lines.push(nums);
        if all_lines.len() == 3 {
            max_correct_answer = all_lines[0].parse::<i32>().unwrap();
            let friend_answer: Vec<char> = all_lines[2].chars().collect();
            let your_answer: Vec<char> = all_lines[1].chars().collect();

            for iterator in 0..friend_answer.len() {
                if friend_answer[iterator] == your_answer[iterator] {
                    count_same += 1;
                } else if friend_answer[iterator] != your_answer[iterator]{
                    count_different += 1;
                }
            }
            println!("{}", count_same + count_different - (count_same - max_correct_answer).abs());
        }
    }
}
