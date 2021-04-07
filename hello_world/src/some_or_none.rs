/// # Examples
/// ```
/// use hello_world::unwrap_some_or_none;
/// let c = unwrap_some_or_none(None);
/// assert_eq!(c, "Toto");
/// ```
/// ```
/// use hello_world::unwrap_some_or_none;
/// let d = unwrap_some_or_none(Some("hehe"));
/// assert_eq!(d, "hehe");
/// ```
/// ```
/// use hello_world::unwrap_some_or_else;
/// let e = unwrap_some_or_else(None);
/// assert_eq!(e, "The Dog");
/// ```

pub fn unwrap_some_or_none_with_error(input: Option<&'static str>) -> &str {
    input.unwrap_or(0) 
}

pub fn unwrap_some_or_none(input: Option<&'static str>) -> String {
    input.unwrap_or("Toto").to_string()
}

pub fn unwrap_some_or_else(input: Option<&'static str>) -> &str {
    input.unwrap_or_else(|| { "The Dog" })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_some_or_none() {
        assert_eq!(unwrap_some_or_none(None), "Toto");
        assert_eq!(unwrap_some_or_none(Some("Hoho")), "Hoho");
    }

    #[test]

    fn test_some_or_else() {
        assert_eq!(unwrap_some_or_else(None), "The Dog");
    }
}