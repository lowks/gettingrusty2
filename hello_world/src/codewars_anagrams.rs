#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        do_test("abba", &["aabb", "abcd", "bbaa", "dada"], &["aabb", "bbaa"]);
        
        do_test(
            "racer",
            &["crazer", "carer", "racar", "caers", "racer"],
            &["carer", "racer"],
        );
    }

    fn do_test(word: &str, words: &[&str], exp: &[&str]) {
        let words: Vec<String> = words.iter().map(|w| w.to_string()).collect();
        let expected: Vec<String> = exp.iter().map(|w| w.to_string()).collect();
        let got = anagrams(word, &words);
        assert_eq!(
            got, expected,
            "Failed with word: \"{}\"\nwords: {:#?}",
            word, words
        );
    }
}

pub fn anagrams(word: &str, words: &[String]) -> Vec<String> {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));

    words.to_owned().iter().map(|x| x.to_string()).filter(|x| {
       let mut chars2: Vec<char> = x.chars().collect();
       chars2.sort_by(|a, b| b.cmp(a));
       chars2 == chars
    }).collect::<Vec<String>>()
}