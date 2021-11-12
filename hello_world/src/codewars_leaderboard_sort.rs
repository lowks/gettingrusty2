#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            leaderboard_sort(
                &["John".into(), "Brian".into(), "Jim".into(), "Dave".into(), "Fred".into()],
                &["Dave +1".into(), "Fred +4".into(), "Brian -1".into()],
            ),
            &["Fred", "John", "Dave", "Brian", "Jim"],
        );
        assert_eq!(
            leaderboard_sort(
                &["Bob".into(), "Larry".into(), "Kevin".into(), "Jack".into(), "Max".into()],
                &["Max +3".into(), "Kevin -1".into(), "Kevin +3".into()],
            ),
            &["Bob", "Kevin", "Max", "Larry", "Jack"],
        );
    }
}

pub fn leaderboard_sort(leaderboard: &[String], changes: &[String]) -> Vec<String> {
//         let mut v = leaderboard.clone().to_vec();
        let mut v = &(*leaderboard).clone().to_vec();
        changes.iter().map(|x| {
            let mut split = x.split(" ");
            let ns_tuple = (split.next().unwrap(), split.next().unwrap());
            let position = v.iter().position(|x| *x == ns_tuple.0).unwrap().to_string().parse::<i32>().unwrap();
            v.remove(position as usize);
            v.insert((position-ns_tuple.1.to_string().parse::<i32>().unwrap()) as usize, ns_tuple.0.to_string());
        }).for_each(drop);
        v
    }
