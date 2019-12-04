// Problem 2
// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.

// Just test each number to see if it's even
fn naive(max: u64) -> u64 {
    let mut sum: u64 = 0;

    let (mut x1, mut x2) = (1u64, 2u64);
    while x2 <= max {
        if x2 % 2 == 0 {
            sum += x2;
        }

        let tmp = x1 + x2;
        x1 = x2;
        x2 = tmp;
        // Apparently you can't just do this: (x1, x2) = (x2, x1 + x2);
    }

    sum
}

// Every third number is even (starting with the second term).
// Given even term x2 and its predecessor x1, the next even term x5 and its predecessor x4 can be calculated:
// x3 = x1 + x2 = 1*x1 + 1*x2
// x4 = x3 + x2 = 1*x1 + 2*x2
// x5 = x4 + x3 = 2*x1 + 3*x2
// Then replace x1 and x2 with x4 and x5 and repeat until x5 is larger than the max.
fn optimized_1(max: u64) -> u64 {
    let mut sum: u64 = 0;

    let (mut x1, mut x2) = (1u64, 2u64);
    while x2 <= max {
        sum += x2;
        let x4 = 1 * x1 + 2 * x2;
        let x5 = 2 * x1 + 3 * x2;
        x1 = x4;
        x2 = x5;
    }

    sum
}

pub fn solve() {
    println!("Naive: {}", naive(4000000));
    println!("Optimized 1: {}", optimized_1(4000000));
}

#[cfg(test)]
mod test {
    #[test]
    fn test_example() {
        assert_eq!(super::naive(1), 0);
        assert_eq!(super::naive(2), 2);
        assert_eq!(super::naive(3), 2);
        assert_eq!(super::naive(5), 2);
        assert_eq!(super::naive(8), 10);
        assert_eq!(super::naive(13), 10);
        assert_eq!(super::naive(21), 10);
        assert_eq!(super::naive(34), 44);
        assert_eq!(super::naive(55), 44);
        assert_eq!(super::naive(89), 44);

        assert_eq!(super::optimized_1(1), 0);
        assert_eq!(super::optimized_1(2), 2);
        assert_eq!(super::optimized_1(3), 2);
        assert_eq!(super::optimized_1(5), 2);
        assert_eq!(super::optimized_1(8), 10);
        assert_eq!(super::optimized_1(13), 10);
        assert_eq!(super::optimized_1(21), 10);
        assert_eq!(super::optimized_1(34), 44);
        assert_eq!(super::optimized_1(55), 44);
        assert_eq!(super::optimized_1(89), 44);
    }

    #[test]
    fn test_answer() {
        assert_eq!(super::naive(4000000), 4613732);
        assert_eq!(super::optimized_1(4000000), 4613732);
    }
}