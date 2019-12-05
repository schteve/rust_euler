// Problem 5
// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

use std::cmp;

// Take each number starting with 1, and see if each number divides it evenly.
fn naive(range_max: u32) -> u32 {
    let mut smallest_multiple = 1;

    loop {
        let mut success: bool = true;

        for i in 1..range_max {
            if smallest_multiple % i != 0 {
                success = false;
                break;
            }
        }

        if success == false {
            smallest_multiple += 1;
        } else {
            return smallest_multiple;
        }
    }
}

// Find the maximum number of each prime factor for each number in the range.
// Then, calculate the product of the prime factors -- this is the smallest number
// that all other numbers divide evenly.
fn optimized_1(range_max: u32) -> u64 {
    let mut factors: Vec<u32> = vec![0; range_max as usize];

    for i in 1..range_max {
        let i_factors = prime_factors(i);
        for idx in 0..i_factors.len() as usize {
            factors[idx] = cmp::max(factors[idx], i_factors[idx]);
        }
    }

    let mut multiple = 1u64;
    for (idx, &count) in factors.iter().enumerate() {
        multiple *= (idx as u32).pow(count) as u64;
    }

    multiple
}

// Starting with 2, try each possible factor to see if it divides evenly.
// Count the number of times the factor divides and push it into the return vector.
// This is inefficient: non-prime divisors are checked.
fn prime_factors(mut number: u32) -> Vec<u32> {
    let mut factors: Vec<u32> = vec![0, 0]; // 0 and 1 cannot be factors

    let mut factor: u32 = 2;
    while factor <= number {
        let mut count = 0;
        while (number % factor) == 0 {
            // Factor evenly divides number, and it is guaranteed prime
            // since we have tried every prime factor less than it.
            // println!("{} divides {} -> {}", factor, number, number / factor);
            number /= factor;
            count += 1;
        }

        factor += 1;
        factors.push(count);
    }

    factors
}

pub fn solve() {
    // println!("Naive: {}", naive(20));
    println!("Optimized 1: {}", optimized_1(20));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(naive(10), 2520);
        assert_eq!(optimized_1(10), 2520);
    }

    #[test]
    fn test_answer() {
        assert_eq!(naive(20), 232792560);
        assert_eq!(optimized_1(20), 232792560);
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(2), [0, 0, 1]);
        assert_eq!(prime_factors(3), [0, 0, 0, 1]);
        assert_eq!(prime_factors(4), [0, 0, 2]);
        assert_eq!(prime_factors(5), [0, 0, 0, 0, 0, 1]);
        assert_eq!(prime_factors(6), [0, 0, 1, 1]);
        assert_eq!(prime_factors(100), [0, 0, 2, 0, 0, 2]);
    }
}
