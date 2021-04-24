pub fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }

    println!("Some refreshing {} is all I need.", beverage);
}

#[test]
#[should_panic]

fn test_should_panic() {
    drink("lemonade");
}
