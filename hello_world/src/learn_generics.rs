use std::ops::Add;
use std::ops::Mul;

fn square<T: Mul<Output = T> + Copy>(x: T) -> T {
    x * x
}

fn add<T>(x: T, y: T) -> T
where
    T: Add + Add<Output = T>,
{
    x + y
}

fn return_number<MyType>(number: MyType) -> MyType {
    println!("Here is your number.");
    number
}

fn compare_and_return_string<T, U>(statement1: T, statement2: T, num_1: U, num_2: U) -> T
where
    U: PartialOrd,
{
    if num_1 > num_2 {
        statement1
    } else {
        statement2
    }
}

#[test]

fn test_square() {
    assert_eq!(square(5), 25)
}

#[test]
fn test_return_number() {
    assert_eq!(return_number(5), 5);
    assert_eq!(return_number(5.5), 5.5);
    assert_eq!(return_number("hello"), "hello")
}

#[test]
fn test_add() {
    assert_eq!(add(2, 5), 7);
    assert_eq!(add(2.2, 3.5), 5.7);
}
