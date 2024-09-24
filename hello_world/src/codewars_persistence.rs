#[cfg(test)]
mod tests {

    #[test]
    fn sample_tests() {
        assert_eq!(super::persistence(39), 3);
        assert_eq!(super::persistence(4), 0);
        assert_eq!(super::persistence(25), 2);
        assert_eq!(super::persistence(999), 4);
    }
}

pub fn persistence(num: u64) -> u64 {
    let mut count = 0;
    let mut product: u64 = num;
    while product.to_string().len() > 1 {
      count += 1;
      product = product.to_string()
                 .chars()
                 .map(|d| d.to_string().parse::<u64>().unwrap())
                 .collect::<Vec<_>>()
                 .iter()
                 .fold(1,|a, &b| (a*b) as u64);
     }
    count
}
