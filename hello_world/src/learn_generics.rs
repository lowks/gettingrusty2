use std::ops::Mul;

fn square<T: Mul<Output = T> + Copy> (x: T) -> T {
    return x * x;
}

#[test]

fn test_square() {
    assert_eq!(square(5), 25)
}