#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_match_country_code() {
        assert_eq!(match_country(45), "UK");
    }

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(45), false);
    }

    #[test]
    fn test_is_bigger_than() {
        assert_eq!(bigger_than(13), false);
    }

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(fizz_buzz(3), "fizz");
        assert_eq!(fizz_buzz(11), "11");
        assert_eq!(fizz_buzz(49), "49");
        assert_eq!(fizz_buzz(20), "buzz");
    }
}

pub fn bigger_than(number: i32) -> bool {
    if number < 11 {
        true
    } else {
        false
    }
}

pub fn match_country(country_code: i32) -> &'static str {
    let country = match country_code {
        45 => "UK",
        43 => "MY",
        60 => "US",
        1..=200 => "Unknown",
        _ => "Invalid",
    };
    country
}

pub fn is_even(input: i64) -> bool {
    match input % 2 {
        0 => true,
        _ => false,
    }
}

pub fn fizz_buzz(input_number: i32) -> String {
    // let input_number_string = input_number.to_string().to_owned();
    let return_str = match (input_number % 3 == 0, input_number % 5 == 0) {
        (true, true) => "fizzbuzz".to_string(),
        (true, false) => "fizz".to_string(),
        (false, true) => "buzz".to_string(),
        (false, false) => input_number.to_string(),
    };
    return_str
}
