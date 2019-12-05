// Problem 4
// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
// Find the largest palindrome made from the product of two 3-digit numbers.

use std::cmp;

// Loop over every factor and generate every product, and test if it is a palindrome.
fn naive(max_factor: u32) -> u32 {
    let mut largest_palindrome = 1;

    for i in 1..max_factor {
        for j in 1..max_factor {
            let product = i * j;
            if is_palindrome(product) == true {
                largest_palindrome = cmp::max(product, largest_palindrome);
            }
        }
    }

    largest_palindrome
}

// Produce a string from the number, and check for matching character pairs from the start and end.
fn is_palindrome(number: u32) -> bool {
    let s = number.to_string();
    let b = s.as_bytes();

    let (mut i, mut j) = (0, s.len() - 1);
    loop {
        if b[i] != b[j] {
            return false;
        } else if i >= j {
            break;
        } else {
            i += 1;
            j -= 1;
        }
    }

    // println!("{}", s);

    true
}

pub fn solve() {
    println!("Naive: {}", naive(999));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(naive(4), 9);
        assert_eq!(naive(100), 9009);
    }

    #[test]
    fn test_answer() {
        assert_eq!(naive(999), 906609);
    }

    #[test]
    fn test_palindrome() {
        assert_eq!(is_palindrome(1), true);
        assert_eq!(is_palindrome(11), true);
        assert_eq!(is_palindrome(12), false);
        assert_eq!(is_palindrome(111), true);
        assert_eq!(is_palindrome(123), false);
        assert_eq!(is_palindrome(1234554321), true);
    }
}
