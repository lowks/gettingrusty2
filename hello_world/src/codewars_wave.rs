#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let expect = ["Hello", "hEllo", "heLlo", "helLo", "hellO"];
        assert_eq!(wave("hello"), expect);
        
        let expect = ["Codewars", "cOdewars", "coDewars", "codEwars", "codeWars", "codewArs", "codewaRs", "codewarS"];
        assert_eq!(wave("codewars"), expect);
        
        let expect: [&str; 0] = [];
        assert_eq!(wave(""), expect);
        
        let expect = ["Two words", "tWo words", "twO words", "two Words", "two wOrds", "two woRds", "two worDs", "two wordS"];
        assert_eq!(wave("two words"), expect);
        
        let expect = [" Gap ", " gAp ", " gaP "];
        assert_eq!(wave(" gap "), expect);
    }
}

pub fn wave(s: &str) -> Vec<String> {
    let mut output_vec2 = Vec::new();
    for (index, char) in s.chars().enumerate() {
        let mut word = &mut &(*s).clone().to_string();
        if char != ' ' {
            word.replace_range(index..index+1, &char.to_string().to_uppercase());
            output_vec2.push(word.clone());
        }
    }
    output_vec2
}

pub fn wave_other(s: &str) -> Vec<String> {
    s.char_indices()
        .map(|(i, c)| s[..i].to_string() + &c.to_uppercase().to_string() + &s[i + 1..])
        .filter(|wave| wave != s)
        .collect()
}