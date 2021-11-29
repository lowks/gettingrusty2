#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(shifter("ON"), 1, "Wrong result for input \"ON\"");
        assert_eq!(shifter("OS IS UPDATED"), 2, "Wrong result for input \"OS IS UPDATED\"");
        assert_eq!(shifter("WHO IS WHO"), 2, "Wrong result for input \"WHO IS WHO\"");
        assert_eq!(shifter("JS"), 0, "Wrong result for input \"JS\"");
        assert_eq!(shifter("I III I III"), 2, "Wrong result for input \"I III I III\"");
        assert_eq!(shifter(""), 0, "Wrong result for input \"\"");
    }
}

fn shifter(s: &str) -> usize {
    let s_split = String::from(s);
    let mut s_split_dedup = s_split.split(' ').collect::<Vec<&str>>();
    s_split_dedup.sort();
    s_split_dedup.dedup();
    let mut _output = Vec::new();
    let mut count = 0;
    _output = s_split_dedup.clone().iter().map(|x| {
        if x.chars().all(|x| ['H', 'I', 'N', 'O', 'S', 'X', 'Z', 'M', 'W'].contains(&x)) && !x.is_empty() {
            count += 1;
            x
        } else {
            ""
        }
    }).collect::<Vec<&str>>();
    count
}
