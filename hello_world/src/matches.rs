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

    #[test]
    fn test_fizz_buzz2() {
        assert_eq!(fizzbuzz2(3), "fizz");
        assert_eq!(fizzbuzz2(11), "11");
        assert_eq!(fizzbuzz2(49), "49");
        assert_eq!(fizzbuzz2(20), "buzz");
    }

    fn test_match_multiple() {
        assert_eq!(match_multiple("hello"), "found");
    }
}

pub fn bigger_than(number: i32) -> bool {
    number < 11
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
    input % 2 == 0
}

pub fn fizz_buzz(input_number: i32) -> String {
    let mut return_string = String::from("");
    match input_number % 3 == 0 {
        true if input_number % 5 == 0 => return_string += "fizzbuzz",
        true if input_number % 5 != 0 => return_string += "fizz",
        false if input_number % 5 == 0 => return_string += "buzz",
        _ => return_string += &input_number.to_string(),
    };
    return_string
}

pub fn match_multiple(input_string: &str) -> String {
    match input_string {
        "hello" | "world" => "found".to_string(),
        _ => "not found".to_string(),
    }
}

pub fn fizzbuzz2(input_number: i32) -> String {
    let mut return_string = String::from("");
    match input_number {
        input_number if input_number % 15 == 0 => return_string += "fizzbuzz",
        input_number if input_number % 3 == 0 => return_string += "fizz",
        input_number if input_number % 5 == 0 => return_string += "buzz",
        _ => return_string += &input_number.to_string(),
    };
    return_string
}

// pub fn if_let_fizz_buzz(input_number: i32) -> String {
//     let mut return_string = String::from("");
//     if let (input_number % 15 == 0) {

//     }
//     return_string
// }
