#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(high("man i need a taxi up to ubud"), "taxi");               
        assert_eq!(high("what time are we climbing up the volcano"), "volcano");
        assert_eq!(high("take me to semynak"), "semynak");                      
        assert_eq!(high("massage yes massage yes massage"), "massage");         
        assert_eq!(high("take two bintang and a dance please"), "bintang"); 
        assert_eq!(high("aa b"), "aa");         
        assert_eq!(high("b aa"), "b");     
        assert_eq!(high("bb d"), "bb");                            
        assert_eq!(high("d bb"), "d"); 
        assert_eq!(high("aaa b"), "aaa");                                     
    }
}

use std::collections::HashMap;

fn high(input: &str) -> &str {
    let mut alphabets: HashMap<char, i32> = HashMap::new();
    
    for (index, char) in (b'a' ..= b'z').enumerate() {
        alphabets.insert(char::from(char), (index+1).to_string().parse::<i32>().unwrap());
    }
    
    let maximum_numbers:Vec<(&str,i32)> = input.split(" ").map(|x| {
        let mut count = 0;
        for y in x.chars() {
           count += alphabets.get(&y).unwrap();
        }
        (x, count)
    }).collect();

    let maximum_word:Option<&(&str, i32)> = maximum_numbers.iter().max_by(|(_, y), (_, y1)| y.partial_cmp(y1).unwrap());
    
    let max_value = match maximum_word {
        Some(x) => x.1,
        None => 0,
    };
    
    let return_value = maximum_numbers.iter().filter(|(_, y)| *y == max_value).collect::<Vec<_>>();
    
    if return_value.len() > 0 {
        return_value[0].0
    } else {
        &""
    }
}