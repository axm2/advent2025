#![allow(dead_code)]

advent_of_code::solution!(2);

/// Count numbers in [L, R] that are formed by concatenating a k-digit block with itself:
/// N = x * (10^k + 1), where x has exactly k digits (no leading zeros).
///
/// This implementation uses integer arithmetic only (no f64), iterates k as integers,
/// and is safe for the full i64 positive range.
fn pow10_i128(exp: u32) -> i128 {
    let mut v: i128 = 1;
    for _ in 0..exp {
        v *= 10;
    }
    v
}

fn digits_positive_i64(mut n: i64) -> u32 {
    debug_assert!(n > 0);
    let mut d = 0;
    while n > 0 {
        n /= 10;
        d += 1;
    }
    d
}

fn ceil_div_i128(a: i128, b: i128) -> i128 {
    // assume b > 0 and a >= 0
    if a <= 0 {
        0
    } else {
        (a + b - 1) / b
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    // parse input like "123-456,789-1011,..." into pairs
    let pairs: Vec<(i64, i64)> = input
        .trim()
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|r| {
            let mut parts = r.trim().split('-');
            let a = parts.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
            let b = parts.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
            (a.min(b), a.max(b))
        })
        .collect();

    let mut total: i128 = 0;

    for (lo, hi) in pairs {
        if hi < 11 || lo > hi {
            continue;
        }
        // clamp lo to at least 11 (smallest repeated-twice positive integer is 11)
        let lo = lo.max(11);
        let hi = hi;

        // compute maximum k to consider: floor(digits(hi) / 2)
        let max_digits = digits_positive_i64(hi);
        let max_k = (max_digits / 2) as u32;
        if max_k == 0 {
            continue;
        }

        // iterate k = 1..=max_k
        for k in 1..=max_k {
            // multiplier = 10^k + 1
            let ten_pow_k = pow10_i128(k);
            let multiplier = ten_pow_k + 1; // fits easily in i128

            // x must have exactly k digits: x in [10^{k-1}, 10^k - 1]
            let x_min_digits = pow10_i128(k - 1);
            let x_max_digits = ten_pow_k - 1;

            // allowable x by range: ceil(lo / multiplier) .. floor(hi / multiplier)
            let x_min_by_range = ceil_div_i128(lo as i128, multiplier);
            let x_max_by_range = (hi as i128) / multiplier;

            // combine constraints
            let x_low = std::cmp::max(x_min_digits, x_min_by_range);
            let x_high = std::cmp::min(x_max_digits, x_max_by_range);

            if x_low <= x_high {
                total += x_high - x_low + 1;
            }
        }
    }

    Some(total as i64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

/// Compute the sum of numbers in the input ranges that are formed by concatenating a k-digit
/// block with itself (xx). Returns the sum as i128 to avoid overflow for large aggregates.
pub fn part_one_sum(input: &str) -> Option<i128> {
    // parse input like "123-456,789-1011,..." into pairs
    let pairs: Vec<(i64, i64)> = input
        .trim()
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .map(|r| {
            let mut parts = r.trim().split('-');
            let a = parts.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
            let b = parts.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
            (a.min(b), a.max(b))
        })
        .collect();

    let mut total: i128 = 0;

    for (lo, hi) in pairs {
        if hi < 11 || lo > hi {
            continue;
        }
        let lo = lo.max(11);
        let hi = hi;

        let max_digits = digits_positive_i64(hi);
        let max_k = (max_digits / 2) as u32;
        if max_k == 0 {
            continue;
        }

        for k in 1..=max_k {
            let ten_pow_k = pow10_i128(k);
            let multiplier = ten_pow_k + 1;

            let x_min_digits = pow10_i128(k - 1);
            let x_max_digits = ten_pow_k - 1;

            let x_min_by_range = ceil_div_i128(lo as i128, multiplier);
            let x_max_by_range = (hi as i128) / multiplier;

            let x_low = std::cmp::max(x_min_digits, x_min_by_range);
            let x_high = std::cmp::min(x_max_digits, x_max_by_range);

            if x_low <= x_high {
                // sum over x = x_low .. x_high: n * (x_low + x_high) / 2
                let n = x_high - x_low + 1;
                let sum_x = (x_low + x_high) * n / 2;
                total += multiplier * sum_x;
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_ranges() {
        // 11,22,33,...,99 (k=1, x=1..9) -> 9 numbers in [11,99]
        assert_eq!(part_one("11-99"), Some(9));

        // 1010, 1111, ... 9999 (k=2, x=10..99) are 90 numbers in [1010, 9999]
        assert_eq!(part_one("1010-9999"), Some(90));

        // Mixed range covering several ks:
        // For k=1: 11..99 -> 9 numbers
        // For k=2: 1010..9999 -> 90 numbers
        // Total in [11,9999] = 99
        assert_eq!(part_one("11-9999"), Some(99));

        // sums
        // k=1 sum: 11 * (1+...+9) = 11*45 = 495
        assert_eq!(part_one_sum("11-99"), Some(495));
        // k=2 sum: sum_{x=10..99} x * 101 = (10+99)*90/2 * 101 = 4905*101 = 495405
        // total [11-9999] = 495 + 495405 = 495900
        assert_eq!(part_one_sum("11-9999"), Some(495900));
    }

    #[test]
    fn example_values() {
        // 1010 is 10 repeated twice
        assert_eq!(part_one("1010-1010"), Some(1));
        assert_eq!(part_one_sum("1010-1010"), Some(1010));
        // 11851185 is 1185 repeated twice
        assert_eq!(part_one("11851185-11851185"), Some(1));
        assert_eq!(part_one_sum("11851185-11851185"), Some(11851185));
        // 0101 should not be valid as input would parse it as 101
        assert_eq!(part_one("101-101"), Some(0));
        assert_eq!(part_one_sum("101-101"), Some(0));
    }

    #[test]
    fn multiple_pairs() {
        // two disjoint ranges
        assert_eq!(part_one("11-99,1010-1010"), Some(10)); // 9 from first + 1 from second
        assert_eq!(part_one_sum("11-99,1010-1010"), Some(495 + 1010));
    }

    #[test]
    fn test_example_part_one() {
        let result = part_one_sum(&advent_of_code::template::read_file_part("examples", DAY,0));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_one() {
        let result = part_one_sum(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
