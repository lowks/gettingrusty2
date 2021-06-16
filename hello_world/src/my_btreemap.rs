use std::collections::BTreeMap;

#[test]

pub fn test_add_value() {
    let mut input_map: BTreeMap<&str, i32> = ["Alice", "Bob", "Carol", "Cheryl"]
    .iter()
    .map(|&s| (s, 0))
    .collect();
    let mut output_map: BTreeMap<&str, i32> = ["Alice", "Bob", "Carol", "Cheryl"]
    .iter()
    .map(|&s| (s, 50))
    .collect();
    assert_eq!(add_value(input_map, 50), output_map);
}

pub fn add_value(mut input_map: BTreeMap<&str, i32>, add_value: i32) -> BTreeMap<&str, i32> {
    for (_, balance) in input_map.range_mut("A"..="Cheryl") {
        *balance += add_value;
    };
    input_map
}