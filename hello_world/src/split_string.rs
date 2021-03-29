
#[cfg(test)]
mod tests {
    
    use super::*;
    #[test]

    fn test_split() {
        assert_eq!(split_string("Hello World", " "), vec!["Hello", "World"]);
        assert_eq!(split_string("Hello,World", ","), vec!["Hello", "World"]);
        assert_eq!(split_string("Hello World Too", " "), vec!["Hello", "World", "Too"]);
    }

    #[test]

    fn test_join() {
        assert_eq!(join_string(["hello", "world"].to_vec()), "hello world");
    }
}

/// # Examples
/// ```
/// use hello_world::split_string;
/// let c = split_string("Hello World"," ");
/// let c1 = split_string("Hello,World", ",");
/// assert_eq!(c, vec!["Hello", "World"]);
/// assert_eq!(c1, vec!["Hello", "World"]);
/// ```
/// ```
/// use hello_world::join_string;
/// let d = join_string(vec!["hello", "world"]);
/// assert_eq!(d, "hello world");
/// ```

pub fn split_string<'a>(string1: &'a str, split_by: &'a str) -> Vec<&'a str> {
    // println!("split string char is {}", split_by);
    string1.split(split_by).collect()
}

pub fn join_string(strings: Vec<&str>) -> String {
    let it = strings.into_iter();
    let s = it.fold(String::new(), |a, b| a + b + " ");
    s.trim().to_owned()
}