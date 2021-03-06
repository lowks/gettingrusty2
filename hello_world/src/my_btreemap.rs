use std::collections::BTreeMap;

#[test]

pub fn test_add_value() {
    let input_map: BTreeMap<&str, i32> = ["Alice", "Bob", "Carol", "Cheryl"]
    .iter()
    .map(|&s| (s, 0))
    .collect();
    let output_map: BTreeMap<&str, i32> = ["Alice", "Bob", "Carol", "Cheryl"]
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

#[test]

pub fn test_iter_mut() {
    let mut input_map  = BTreeMap::new();
    input_map.insert("a", 1);
    input_map.insert("b", 2);
    let mut output_map = BTreeMap::new();
    output_map.insert("a", 11);
    output_map.insert("b", 12);
    assert_eq!(btreemap_iter_mut(input_map, 10), output_map);
}

#[test]

pub fn test_append_to_all() {
    let mut input_map  = BTreeMap::new();
    input_map.insert(1, String::from("hello"));
    input_map.insert(2, String::from("hello"));
    let mut output_map = BTreeMap::new();
    output_map.insert(1, String::from("hello world"));
    output_map.insert(2, String::from("hello world"));
    assert_eq!(append_to_all(input_map, " world"), output_map);
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

pub fn btreemap_iter_mut(mut input_map: BTreeMap<&str, i32>, add_by: i32) -> BTreeMap<&str, i32> {
    for (key, value) in input_map.iter_mut() {
        *value += add_by;
    };
    input_map
}

pub fn append_to_all(mut input_map: BTreeMap<i32, String>, append_string: &str) -> BTreeMap<i32, String> {
    for string in input_map.values_mut() {
        string.push_str(append_string)
    };
    input_map
}