#[test]
fn test_1() {
    assert_eq!("1", raindrops(1));
}

#[test]
fn test_3() {
    assert_eq!("Pling", raindrops(3));
}

#[test]
fn test_5() {
    assert_eq!("Plang", raindrops(5));
}

#[test]
fn test_7() {
    assert_eq!("Plong", raindrops(7));
}

#[test]
fn test_6() {
    assert_eq!("Pling", raindrops(6));
}

#[test]
fn test_8() {
    assert_eq!("8", raindrops(8));
}

#[test]
fn test_9() {
    assert_eq!("Pling", raindrops(9));
}

#[test]
fn test_10() {
    assert_eq!("Plang", raindrops(10));
}

#[test]
fn test_14() {
    assert_eq!("Plong", raindrops(14));
}

#[test]
fn test_15() {
    assert_eq!("PlingPlang", raindrops(15));
}

#[test]
fn test_21() {
    assert_eq!("PlingPlong", raindrops(21));
}

#[test]
fn test_25() {
    assert_eq!("Plang", raindrops(25));
}

#[test]
fn test_27() {
    assert_eq!("Pling", raindrops(27));
}

#[test]
fn test_35() {
    assert_eq!("PlangPlong", raindrops(35));
}

#[test]
fn test_49() {
    assert_eq!("Plong", raindrops(49));
}

#[test]
fn test_52() {
    assert_eq!("52", raindrops(52));
}

#[test]
fn test_105() {
    assert_eq!("PlingPlangPlong", raindrops(105));
}

#[test]
fn test_3125() {
    assert_eq!("Plang", raindrops(3125));
}

#[test]
fn test_12121() {
    assert_eq!("12121", raindrops(12_121));
}

pub fn raindrops(n: u32) -> String {
    use std::collections::BTreeMap;

    let mut result = String::new();
    let mut drops = BTreeMap::new();

    drops.insert(3, "Pling");
    drops.insert(5, "Plang");
    drops.insert(7, "Plong");

    for (key, value) in drops {
        if n % key == 0 {
            result += value
        };
    }

    if result.is_empty() {
        result += &n.to_string()
    };
    result.to_owned()
}
