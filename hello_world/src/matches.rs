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
        assert_eq!(bigger_than(13), true);
    }
}

pub fn bigger_than(number: i32) -> bool {
     if let j = (number < 11) {
        return true 
    } else {
        return false
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
        _ => false
    }
}
