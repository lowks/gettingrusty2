/// # Examples
/// ```
/// use hello_world::okor;
/// let c = okor(None);
/// let d = okor(Some("Dog"));
/// assert_eq!(c, Err(0));
/// assert_eq!(d, Ok("Dog"));
/// assert!(c.is_err());
/// assert!(d.is_ok());
/// ```

pub fn okor(input: Option<&'static str>) -> Result<&str, i32> {
    input.ok_or(0)
}

#[test]

fn test_some_or_else() {
    assert_eq!(okor(None), Err(0));
    assert_eq!(okor(Some("Dog")), Ok("Dog"));
    let bad_result = okor(None);
    let good_result = okor(Some("Dog"));
    assert!(bad_result.is_err());
    assert!(good_result.is_ok());
}
