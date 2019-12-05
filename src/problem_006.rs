// Problem 6
// The sum of the squares of the first ten natural numbers is,
//      12 + 22 + ... + 102 = 385
// The square of the sum of the first ten natural numbers is,
//      (1 + 2 + ... + 10)^2 = 552 = 3025
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

// Calculate the sum of squares, then the square of sums, then subtract them.
fn naive(max: u32) -> u32 {
    let mut sum_of_squares = 0u32;
    for i in 1..(max+1) {
        let square = i * i;
        sum_of_squares += square;
    }

    let mut sum = 0u32;
    for i in 1..(max+1) {
        sum += i;
    }
    let square_of_sums = sum * sum;

    let difference = square_of_sums - sum_of_squares;
    difference
}

pub fn solve() {
    println!("Naive: {}", naive(100));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(naive(10), 2640);
    }

    #[test]
    fn test_answer() {
        assert_eq!(naive(100), 25164150);
    }
}
