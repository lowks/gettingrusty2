use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let date_string = &line;

        match date_string.as_str() {
            "OCT 31" | "DEC 25" => println!("yup"),
            _ => println!("nope"),
        }
        
  }
}
