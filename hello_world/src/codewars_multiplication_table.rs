// Your task, is to create NxN multiplication table, of size provided in parameter.

// for example, when given size is 3:

// 1 2 3
// 2 4 6
// 3 6 9
// for given example, the return value should be: [[1,2,3],[2,4,6],[3,6,9]]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(multiplication_table(3), [[1,2,3], [2,4,6], [3,6,9]]);
    }
}


pub fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    let mut output_vector: Vec<Vec<usize>> = Vec::new();
    let mut iterator:usize = 1;
    while output_vector.len() < len {
        let vec = (1..=len)
                  .collect::<Vec<usize>>()
                  .iter()
                  .map(|x| x * iterator)
                  .collect::<Vec<usize>>();
        output_vector.push(vec);
        iterator += 1;
    }
    output_vector
}
