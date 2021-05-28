/// # Examples
/// ```
/// use hello_world::add_hundred;
/// let c = add_hundred(1);
/// assert_eq!(c, 101);
/// ```
/// ```
/// use hello_world::closure_inferred;
/// let d = closure_inferred(3, 4);
/// assert_eq!(d, 7);
/// ```
/// ```
/// use hello_world::multiply_until;
/// let e = multiply_until(20);
/// assert_eq!(e, 121645100408832000);
/// ```

pub fn add_hundred(i: i64) -> i64 {
    i + 100
}
pub fn closure_inferred(i: i64, j: i64) -> i64 {
    i + j
}
pub fn closure_as_argument<F: Fn(i64) -> i64>(i: i64, f: F) -> i64 {
    f(i) * 3
}
pub fn multiply_until(i: u128) -> u128 {
    // let learn: Vec<u128> = (1..i).map(|v| v).collect();
    let learn: Vec<u128> = (1..i).collect();
    let mut_sum = learn.iter().product::<u128>();
    mut_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_hundred() {
        assert_eq!(add_hundred(10), 110)
    }

    #[test]
    fn test_closure_inferred() {
        assert_eq!(closure_inferred(1, 3), 4)
    }

    #[test]
    fn test_closure_inferred2() {
        assert_eq!(closure_as_argument(10, add_hundred), 330)
    }

    #[test]
    fn test_mut_until() {
        assert_eq!(multiply_until(20), 121645100408832000)
    }
}
