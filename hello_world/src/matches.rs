#[test]

fn test_match_country_code() {
    assert_eq!(match_country(45), "UK");
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
