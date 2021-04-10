/// # Examples
/// ```
/// use hello_world::okor;
/// let c = okor(None);
/// assert_eq!(c, Err(0));
/// ```

pub fn okor(input: Option<&'static str>) -> Result<&str, i32> {
    input.ok_or(0)
}

#[test]

 fn test_some_or_else() {
     assert_eq!(okor(None), Err(0));
}

