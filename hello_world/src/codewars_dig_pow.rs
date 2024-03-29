#[test]

fn test_dig_pow_found() {
    assert_eq!(dig_pow(89, 1), 1);
}

#[test]

fn test_dig_pow_not_found() {
    assert_eq!(dig_pow(92, 1), -1);
}

// Some numbers have funny properties. For example:

// 89 --> 8¹ + 9² = 89 * 1

// 695 --> 6² + 9³ + 5⁴= 1390 = 695 * 2

// 46288 --> 4³ + 6⁴+ 2⁵ + 8⁶ + 8⁷ = 2360688 = 46288 * 51

// Given a positive integer n written as abcd... (a, b, c, d... being digits) and a positive integer p

// we want to find a positive integer k, if it exists, such as the sum of the digits of n taken to the successive powers of p is equal to k * n.
// In other words:

// Is there an integer k such as : (a ^ p + b ^ (p+1) + c ^(p+2) + d ^ (p+3) + ...) = n * k

// If it is the case we will return k, if not return -1.

// Note: n and p will always be given as strictly positive integers.

pub fn dig_pow(n: i64, p: i32) -> i64 {
    let digits = n.to_string().chars()
        .map(|d| d.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();
    let sum = digits.iter()
        .enumerate()
        .map(|(a,b)| {
            let power = a.to_string().parse::<i64>().unwrap();
            i64::pow(b.clone(), (power + p as i64) as u32)
        })
        .collect::<Vec<_>>().iter()
        .sum::<i64>();
        if sum % n == 0 {
            sum / n
        } else {
            -1
        }
}