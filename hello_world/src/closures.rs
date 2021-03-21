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

pub fn add_hundred (i: i64) -> i64 {i + 100}
pub fn closure_inferred  (i:i64, j:i64) -> i64 {i + j}
pub fn closure_as_argument <F: Fn(i64) -> i64> (i: i64, f:F) -> i64{
    f(i) * 3
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_hundred() { assert_eq!(add_hundred(10), 110)}
    
    #[test]
    fn test_closure_inferred() {assert_eq!(closure_inferred(1, 3), 4)}

    #[test]
    fn test_closure_inferred2() {
        assert_eq!(closure_as_argument(10, add_hundred), 330)
    }
}