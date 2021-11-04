#[test]
fn run_tests() {
  assert_eq!(duplicate_encode("din"),"(((");
  assert_eq!(duplicate_encode("recede"),"()()()");
  assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
  assert_eq!(duplicate_encode("(( @"),"))((");
}

fn duplicate_encode(word:&str)->String {
    word.to_ascii_lowercase().chars().fold("".to_string(), |acc, x| 
        match word.to_ascii_lowercase().matches(&x.to_string()).count() {
             1 | 0 => format!("{}{}", acc, "("),
             _ => format!("{}{}", acc, ")"),
    })
}