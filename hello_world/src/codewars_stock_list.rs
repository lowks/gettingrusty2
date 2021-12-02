#[test] 

fn test_stock_list() {
    let b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
    let c = vec!["A", "B"];
    let empty_b = vec![];
    let empty_c = vec![];

    assert_eq!(stock_list(b.clone(), c.clone()), "(A : 200) - (B : 1140)");
    assert_eq!(stock_list(empty_b, c), "");
    assert_eq!(stock_list(b, empty_c), "");
}


pub fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.is_empty() || list_cat.is_empty() {
        "".to_string()  
    } else {
        list_cat.iter().map(|n| {
            let mut count = 0;
            for item in &list_art {
                if item.starts_with(n) {
                    count += item.split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
                }
            };
            format!("({} : {})", n, count)
        }).collect::<Vec<_>>().join(" - ")
    }
}
