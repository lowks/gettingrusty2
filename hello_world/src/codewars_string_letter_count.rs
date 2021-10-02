#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(string_letter_count(&"The quick brown fox jumps over the lazy dog."), "1a1b1c1d3e1f1g2h1i1j1k1l1m1n4o1p1q2r1s2t2u1v1w1x1y1z");
        assert_eq!(string_letter_count(&"The time you enjoy wasting is not wasted time."), "2a1d5e1g1h4i1j2m3n3o3s6t1u2w2y");
        assert_eq!(string_letter_count(&"./4592#{}()"), "");
    }
}

pub fn string_letter_count(s: &str) -> String {
   let lower_case = s.clone().to_ascii_lowercase();
   let mut input_word_lowercase: Vec<char> = lower_case.chars().collect();
   input_word_lowercase.sort_by(|a, b| b.cmp(a));
   input_word_lowercase.reverse();
   let output_chars: Vec<(&char, usize)> = input_word_lowercase.iter().map(|x| {
        let count = lower_case.matches(*x).count();
        (x, count)
  }).collect();
  let folded = output_chars.iter().fold("".to_string(), |mut acc, x| {
      if !acc.contains(*x.0) && (*x.0).is_alphabetic() {
         acc.push_str(&format!("{}{}", x.1, x.0))   
      }
      acc
  });
  folded
}