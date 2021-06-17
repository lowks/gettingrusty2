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

#[test]

pub fn test_split_off() {
    let mut input_map: BTreeMap<i32, &str> = BTreeMap::new();
    input_map.insert(1, "a");
    input_map.insert(2, "b");
    input_map.insert(3, "c");
    input_map.insert(4, "d");
    let mut output_map: BTreeMap<i32, &str> = BTreeMap::new();
    output_map.insert(1, "a");
    output_map.insert(2, "b");
    assert_eq!(split_off(input_map, 3), output_map);
}

pub fn add_value(mut input_map: BTreeMap<&str, i32>, add_value: i32) -> BTreeMap<&str, i32> {
    for (_, balance) in input_map.range_mut("A"..="Cheryl") {
        *balance += add_value;
    };
    input_map
}

pub fn split_off(mut input_map: BTreeMap<i32, &str>, cut_off: i32) -> BTreeMap<i32, &str> {
    input_map.split_off(&cut_off);
    input_map
}