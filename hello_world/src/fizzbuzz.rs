#[test]

fn test_fizz_buzz() {
    assert_eq!(determine_fizz_buzz(4), format!("{}", 4));
    assert_eq!(determine_fizz_buzz(3), "fizz");
    assert_eq!(determine_fizz_buzz(5), "buzz");
    assert_eq!(determine_fizz_buzz(15), "fizz buzz");
}

pub fn determine_fizz_buzz(input: i64) -> String {
    let result = match (input % 3, input % 5) {
        (0, 0) => String::from("fizz buzz"),
        (0, _) => String::from("fizz"),
        (_, 0) => String::from("buzz"),
        (_, _) => format!("{}", input),
    };
    result
}
