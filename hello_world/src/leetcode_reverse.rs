mod tests {
    use super::*;
    
    #[test]
    fn test_reverse_positive_number() {
        assert_eq!(reverse(45), 54);
    }

    #[test]
    fn test_reverse_negative_number() {
        assert_eq!(reverse(-231), -132);
    }

}

pub fn reverse(x: i32) -> i32 {
    let mut output = x.abs().to_string()
                    .chars()
                    .map(|d| d.to_string())
                    .rev()
                    .collect::<Vec<_>>();
    
    match (output.join("").parse::<i32>()) {
        Ok(n) => {
            if x < 0 && n != 0 {
                -n
            } else {
                n
            }
        }
        Err(n) => 0,
    } 
}