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

pub fn unwrap_some_or_none(input: Option<&'static str>) -> String {
    input.unwrap_or("Toto").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_match_country_code() {
        assert_eq!(unwrap_some_or_none(None), "Toto");
        assert_eq!(unwrap_some_or_none(Some("Hoho")), "Hoho");
    }
}