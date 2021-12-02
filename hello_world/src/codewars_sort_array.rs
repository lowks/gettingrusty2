#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
        assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
        assert_eq!(sort_array(&[]), []);
    }
}

pub fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut input_vec = arr.to_owned();
    let mut original_odd_index = Vec::new();
    let odd_vec = input_vec.clone();
    
    for (index, number) in input_vec.iter().enumerate() {
        if number % 2 == 1 {
            original_odd_index.push(index)
        }
    }
    let mut odd_vec2 = odd_vec.iter().filter(|x| *x % 2 == 1).collect::<Vec<_>>().to_owned();
    odd_vec2.sort();
 
 
    for (x,y) in original_odd_index.iter().zip(&odd_vec2) {
        input_vec.remove(*x);
        input_vec.insert(*x, **y);
    }
    input_vec
 }
