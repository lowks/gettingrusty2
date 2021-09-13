#[cfg(test)]
mod tests {
    use super::disemvowel;
    
    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
}

fn disemvowel(s: &str) -> String {
    let return_string = s.replace(&['a','A', 'e','E','i','I','o','O','u','U'][..], "");
    return_string
}