// Problem 3
// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

// Starting with 2, try each possible factor to see if it divides evenly.
// If it does, divide the number and try again. If it doesn't, increment the factor.
// This is inefficient: non-prime divisors are checked.
fn naive(mut number: u64) -> u32 {
    let mut factor: u64 = 2;
    while factor < number {
        if (number % factor) == 0 {
            // Factor evenly divides number, and it is guaranteed prime
            // since we have tried every prime factor less than it.
            // println!("{} divides {} -> {}", factor, number, number / factor);
            number /= factor;
        } else {
            // Factor does NOT evenly divide the number, so increment it and try again.
            factor += 1;
        }
    }

    factor as u32
}

pub fn solve() {
    println!("Naive: {}", naive(600851475143));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(naive(11), 11);
        assert_eq!(naive(15), 5);
        assert_eq!(naive(13195), 29);
    }

    #[test]
    fn test_answer() {
        assert_eq!(naive(600851475143), 6857);
    }
}
