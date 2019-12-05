// Problem 7
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
// What is the 10,001st prime number?

// Try each number starting with 2 to see if it's prime.
// Count until the target number of primes is reached.
fn naive(max: u32) -> u32 {
    let mut counter = 0;

    let mut candidate = 1; // First candidate will be 2
    while counter < max {
        candidate += 1;

        // Try divisors from 2 up to square root of the candidate (minor
        // optimization instead of trying all numbers up to the candidate).
        // If anything divides it, it's not prime!
        let mut is_prime = true;
        let limit = (candidate as f64).sqrt() as u32; // sqrt returns a float
        for i in 2..(limit + 1) {
            if candidate % i == 0 {
                // Evenly divisible -- this number is not prime!
                is_prime = false;
                break;
            }
        }

        if is_prime {
            counter += 1;
            // println!("{} is prime #{}", candidate, counter);
        }
    }

    candidate
}

// Same as naive, but only check divisors that are prime.
// To do this, a list of all known primes must be maintained.
fn optimized_1(max: u32) -> u32 {
    let mut primes = Vec::new();

    let mut candidate = 1; // First candidate will be 2
    while (primes.len() as u32) < max {
        candidate += 1;

        // Try all known primes as divisors, up to the square root of the candidate.
        // If anything divides it, it's not prime!
        let mut is_prime = true;
        let limit = (candidate as f64).sqrt() as u32; // sqrt returns a float
        for &prime in primes.iter() {
            if prime > limit {
                break;
            }

            if candidate % prime == 0 {
                // Evenly divisible -- this number is not prime!
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(candidate);
            // println!("{} is prime #{}", candidate, primes.len());
        }
    }

    candidate
}

// Same as optimized 1, but use an idiomatic iterator for the primes list.
fn optimized_2(max: u32) -> u32 {
    let mut primes = Vec::new();

    let mut candidate = 1; // First candidate will be 2
    while (primes.len() as u32) < max {
        candidate += 1;

        // Try all known primes as divisors, up to the square root of the candidate.
        // If anything divides it, it's not prime!
        let mut is_prime = true;
        let limit = (candidate as f64).sqrt() as u32; // sqrt returns a float
        let prime_iter = primes.iter().take_while(|&x| *x <= limit);
        for &prime in prime_iter {
            if candidate % prime == 0 {
                // Evenly divisible -- this number is not prime!
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(candidate);
            // println!("{} is prime #{}", candidate, primes.len());
        }
    }

    candidate
}

pub fn solve() {
    println!("Naive: {}", naive(10001));
    println!("Optimized 1: {}", optimized_1(10001));
    println!("Optimized 2: {}", optimized_2(10001));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(naive(6), 13);
        assert_eq!(optimized_1(6), 13);
        assert_eq!(optimized_2(6), 13);
    }

    #[test]
    fn test_answer() {
        assert_eq!(naive(10001), 104743);
        assert_eq!(optimized_1(10001), 104743);
        assert_eq!(optimized_2(10001), 104743);
    }
}
