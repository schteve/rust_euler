// Problem 1
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

// Just test each number to see if it's a multiple
fn naive(max: u32) -> u64 {
    let mut sum: u64 = 0;

    for x in 1..max as u64 {
        if (x % 3 == 0) || (x % 5 == 0) {
            sum += x;
        }
    }

    sum
}

// Don't test each number, instead do each multiple of 3 then each multiple of 5 (skipping multiples of 3)
fn optimized_1(max: u32) -> u64 {
    let mut sum: u64 = 0;

    for x in (3..max as u64).step_by(3) {
        sum += x;
    }

    // Starting with 5, every 3rd number should be skipped
    let mut i = 0;
    for x in (5..max as u64).step_by(5) {
        if i < 2 {
            sum += x;
            i += 1;
        } else {
            i = 0;
        }
    }

    sum
}

pub fn solve() {
    println!("Naive: {}", naive(1000));
    println!("Optimized 1: {}", optimized_1(1000));

    // Fun fact: if you keep adding 0's to the end of the parameter the sum grows like this:
    // - 233168
    // - 23331668
    // - 2333316668
    // - 233333166668
    // etc
    // println!("Naive: {}", naive(100000000));
    // println!("Optimized 1: {}", optimized_1(100000000));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(naive(10), 23);
        assert_eq!(optimized_1(10), 23);
    }

    #[test]
    fn test_answer() {
        assert_eq!(naive(1000), 233168);
        assert_eq!(optimized_1(1000), 233168);
    }
}
