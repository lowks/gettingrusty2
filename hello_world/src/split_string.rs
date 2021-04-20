
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

    fn test_split_inclusive() {
        assert_eq!(split_inclusive("hello world ", " "), vec!["hello ", "world "])
    }

    #[test]

    fn test_join() {
        assert_eq!(join_string(["hello", "world"].to_vec()), "hello world");
    }

    #[test]

    fn test_reverse_string() {
        assert_eq!(reverse_string("hello world"), "dlrow olleh");
    }

    #[test]

    fn test_similar_strings() {
        assert_eq!(similar_strings("dlrowolleh", "helloworld"), true);
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
/// ```
/// use hello_world::reverse_string;
/// let e = reverse_string("hello");
/// assert_eq!(e, "olleh");
/// ```

pub fn split_string<'a>(string1: &'a str, split_by: &'a str) -> Vec<&'a str> {
    // println!("split string char is {}", split_by);
    string1.split(split_by).collect()
}

pub fn split_inclusive<'a>(string2: &'a str, split_by: &'a str) -> Vec<&'a str> {
    string2.split_inclusive(split_by).collect()
}

pub fn join_string(strings: Vec<&str>) -> String {
    let it = strings.into_iter();
    let s = it.fold(String::new(), |a, b| a + b + " ");
    s.trim().to_owned()
}

pub fn reverse_string(input_string: &str) -> String {
    input_string.chars().rev().collect::<String>()
}

pub fn similar_strings(input_string1: &str, input_string2: &str) -> bool {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    for char in input_string1.chars() {
        vec1.push(char)
    }
    for char in input_string2.chars() {
        vec2.push(char)
    }
    vec1.sort();
    vec2.sort();
    vec1 == vec2
}