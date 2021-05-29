#[test]
fn test_first_prime() {
    assert_eq!(nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(nth(5), 13);
}

#[test]
#[ignore]
fn test_big_prime() {
    assert_eq!(nth(10_000), 104_743);
}

pub fn nth(n: u32) -> u32 {
    let mut target_count_prime = n + 1;
    let mut result_prime_number = 0;
    let mut current_number = 2;
    if target_count_prime == 0 {
        result_prime_number = 2
    } else {
        while 0 < target_count_prime {
            if is_prime(current_number) {
                result_prime_number = current_number;
                target_count_prime -= 1;
            };
            current_number += 1;
        }
    }
    result_prime_number
}

fn is_prime(num: u32) -> bool {
    if num == 0 {
        return false;
    }

    for i in 2..num {
        if num % i == 0 {
            return false;
        }
    }

    true
}
