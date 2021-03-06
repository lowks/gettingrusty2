use std::collections::VecDeque;
// #![feature(hash_drain_filter)]
use std::collections::HashMap;


// pub fn hash_drain_filter(mut input_map: HashMap<i32, i32>) -> Vec<i32> {
//     let drained: HashMap<i32, i32> = input_map.drain_filter(|k, _v| k % 2 == 0).collect();
//     let mut return_vec = drained.keys().copied().collect::<Vec<_>>();
//     return_vec
// }

/// # Examples
/// ```
/// use hello_world::add_number_to_element;
/// let d = add_number_to_element(Ok(3));
/// assert_eq!(d, Ok(20));
/// ```

pub enum Multiple {
    Integer(i32),
    Blob(String),
}

impl std::fmt::Display for Multiple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Multiple::Integer(v) => v.fmt(f),
            Multiple::Blob(v) => v.fmt(f),
        }
    }
}

// #[test]

// fn test_create_vector() {
//     let v = create_vector();
//     let integer = Multiple::Integer(v);
// }

#[test]

pub fn test_add_number_to_element() {
    assert_eq!(add_number_to_element(Ok(3)), Ok(20))
}

#[test]

pub fn test_create_vec_deque() {
    let mut test_deque = create_vec_deque();
    assert_eq!(test_deque.pop_front(), Some(1));
}

#[test]

pub fn test_retain_deque() {
    let test_deque = retain_deque(10, 2);
    assert_eq!(test_deque, [2, 4, 6, 8]);
}

#[test]

pub fn test_rotate_vecdeque() {
    let test_deque_right = rotate_vecdeque("right", 2, VecDeque::from(vec![1, 2, 3]));
    // assert_eq!(test_deque_right, [3,1,2]);
    assert_eq!(test_deque_right, [2, 3, 1]);

    let test_deque_left = rotate_vecdeque("left", 2, VecDeque::from(vec![1, 2, 3]));
    // assert_eq!(test_deque_left, [2,3,1]);
    assert_eq!(test_deque_left, [3, 1, 2]);
}

#[test]

pub fn test_concat_vec() {
    assert_eq!(concat_vec(vec!["hello", "world"]), "helloworld");
}

#[test]

pub fn test_filter_hashmap_by() {
    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(1, 3);
    map.insert(2, 40);
    map.insert(6, 60);
    // let other_map: HashMap<i32, i32> ={ 6:60, 2:40, 0:20};
    let mut map2: HashMap<i32, i32> = HashMap::new();
    // map2.insert(0, 20);
    map2.insert(2, 40);
    map2.insert(6, 60);
    assert_eq!(filter_hashmap_by(map, 2), map2);
}

#[test]

pub fn test_remove_empty() {
    let mut input_map = HashMap::new();
    input_map.insert("", 1);
    input_map.insert("a", 2);
    let mut output_map = HashMap::new();
    output_map.insert("a", 2);
    assert_eq!(remove_empty(input_map), output_map);
}


#[test]

pub fn test_values_mut() {
    let mut input_map = HashMap::new();
    input_map.insert("b", 1);
    input_map.insert("a", 2);
    let mut output_map = HashMap::new();
    output_map.insert("b", 11);
    output_map.insert("a", 12);
    assert_eq!(values_mutable(input_map), output_map);
}

#[test]
pub fn test_change_mut() {
    let mut input_map = HashMap::new();
    input_map.insert(1, "a");
    let mut output_map = HashMap::new();
    output_map.insert(1, "b");
    assert_eq!(change_mut(input_map, 1, "b"), output_map)
}

// #[test]

// pub fn test_hash_drain_filter() {
//     let mut input_map = HashMap::new();
//     input_map.insert(1, 2);
//     input_map.insert(2, 2);
//     input_map.insert(3, 5);
//     input_map.insert(4, 6);
//     // let mut output_map = HashMap::new();
//     // output_map.insert(2, 2);
//     // output_map.insert(4, 6);
//     assert_eq!(hash_drain_filter(input_map), vec![2,4])
// }

#[test]

pub fn test_change_by_iter_mut() {
    let mut input_map2 = HashMap::new();
    input_map2.insert("a", 1);
    input_map2.insert("b", 2);
    let mut output_map2 = HashMap::new();
    output_map2.insert("a", 2);
    output_map2.insert("b", 4);
    assert_eq!(change_by_iter_mut(input_map2), output_map2);
}

pub fn create_vector() -> Vec<Multiple> {
    let v2 = vec![
        Multiple::Integer(100),
        Multiple::Blob("Hello World".to_string()),
    ];
    v2
}

pub fn add_number_to_element(thing: Result<i32, i32>) -> Result<i32, i32> {
    thing
        .and_then(|number| Ok(number + 1))
        .and_then(|number2| Ok(number2 * 5))
}

pub fn create_vec_deque() -> VecDeque<i32> {
    let mut buf = VecDeque::new();
    buf.push_back(1);
    buf.push_back(2);
    buf.push_back(3);
    buf
}

pub fn retain_deque(until: i32, factor_of: i32) -> VecDeque<i32> {
    let mut buf = VecDeque::new();
    buf.extend(1..until);
    buf.retain(|&x| x % factor_of == 0);
    buf
}

pub fn rotate_vecdeque(
    rotate_direction: &str,
    rotate_by: usize,
    mut input_vec: VecDeque<i32>,
) -> VecDeque<i32> {
    // let mut buf = VecDeque::new();
    // let return_buf = match rotate_direction {
    // match rotate_direction == "right" {
    //     true => input_vec.rotate_right(rotate_by),
    //     false => input_vec.rotate_left(rotate_by),
    //     // &_ => input_vec,
    // };
    if rotate_direction == "right" {
        input_vec.rotate_right(rotate_by);
        return input_vec;
    }
    input_vec.rotate_left(rotate_by);
    input_vec
}

pub fn concat_vec(input_vec: Vec<&str>) -> String {
    input_vec.concat()
}

pub fn filter_hashmap_by(mut input_hashmap: HashMap<i32, i32>, input_number: i32) -> HashMap<i32, i32> {
    input_hashmap.retain(|&k, _| k % input_number == 0);
    input_hashmap
}

pub fn remove_empty(mut input_hashmap: HashMap<&str, i32>) -> HashMap<&str, i32> {
    input_hashmap.retain(|&k, _| !k.is_empty());
    input_hashmap
}

pub fn values_mutable(mut input_hashmap: HashMap<&str, i32>) -> HashMap<&str, i32> {
    for value in input_hashmap.values_mut() {
        *value = *value + 10
    };
    input_hashmap
}

pub fn change_mut<'a>(mut input_map: HashMap<i32, &'a str>, key: i32, value: &'a str) -> HashMap<i32, &'a str> {
    if let Some(x) = input_map.get_mut(&key) {
        *x = value;
    };
    input_map
}

// #![feature(hash_drain_filter)]

// pub fn hash_drain_filter(mut input_map: HashMap<i32, i32>) -> Vec<i32> {
//     let drained: HashMap<i32, i32> = input_map.drain_filter(|k, _v| k % 2 == 0).collect();
//     let mut return_vec = drained.keys().copied().collect::<Vec<_>>();
//     return_vec
// }

pub fn change_by_iter_mut(mut input_map: HashMap<&str, i32>) -> HashMap<&str, i32> {
    for (_key, val) in input_map.iter_mut() {
        *val *= 2;
    };
    input_map
}