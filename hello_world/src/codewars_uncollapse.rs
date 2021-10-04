#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(uncollapse("three"), "three".to_string());
        assert_eq!(uncollapse("eightsix"), "eight six".to_string());
        assert_eq!(uncollapse("fivefourseven"), "five four seven".to_string());
        assert_eq!(uncollapse("ninethreesixthree"), "nine three six three".to_string());
        assert_eq!(uncollapse("foursixeighttwofive"), "four six eight two five".to_string());
    }
}

fn uncollapse(digits: &str) -> String {
    //     unimplemented!()
        let string_digits = String::from(digits);
        let output = string_digits.replace("zero", "zero ")
                                  .replace("one", "one ")
                                  .replace("two", "two ")
                                  .replace("three", "three ")
                                  .replace("four", "four ")
                                  .replace("five", "five ")
                                  .replace("six", "six ")
                                  .replace("seven", "seven ")
                                  .replace("eight", "eight ")
                                  .replace("nine", "nine ");
        output.trim().to_string()
}