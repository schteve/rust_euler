// Problem 9
// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a^2 + b^2 = c^2
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 52.
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

// Try every combination of a, b, c and see if it adds to the target value.
fn naive(target_sum: u32) -> u32 {
    for c in 1..1000 {
        for b in 1..c {
            for a in 1..b {
                if (a * a) + (b * b) == (c * c) {
                    if (a + b + c) == target_sum {
                        return a * b * c;
                    }
                }
            }
        }
    }

    0
}

pub fn solve() {
    println!("Naive: {}", naive(1000));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(naive(12), 60); // 3, 4, 5
        assert_eq!(naive(30), 780); // 5, 12, 13
    }

    #[test]
    fn test_answer() {
        assert_eq!(naive(1000), 31875000);
    }
}
