#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(bingo(&[("ABC", 65), ("HGR", 74), ("BYHT", 74)], 2), "Loser!");
        assert_eq!(bingo(&[("ABC", 65), ("HGR", 74), ("BYHT", 74)], 1), "Winner!");
        assert_eq!(bingo(&[("HGTYRE", 74), ("BE", 66), ("JKTY", 74)], 3), "Loser!");
    }
}

fn bingo<S: AsRef<str>>(ticket: &[(S, u8)], win: usize) -> &'static str {
    if ticket.to_owned().iter().filter(|(x, y)|{
        x.as_ref().as_bytes().contains(&*y)
    }).collect::<Vec<&(S,u8)>>()
      .iter().count() as usize >= win {"Winner!"} else {"Loser!"}
}