use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let strings = &line;
        let smiles = vec![":)", ";)", ":-)", ";-)"];
        let mut v: Vec<_> = Vec::new();
    
        for smile in smiles {
            let matches: Vec<_> = strings.match_indices(smile).collect();
            if matches.len() > 0 {
                for inner_item in matches {
                    v.push(inner_item);
                }
            }
        }
    
        v.sort_by_key(|k| k.0);
        for (k, _) in v {
            println!("{}", k);
        }
        
  }
}
