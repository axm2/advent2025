advent_of_code::solution!(2);

pub fn is_valid_id(n:i64) -> bool {
    // turn to string
    let s = n.to_string();
    if s.len() % 2 == 0{
        let mid = s.len()/2;
        let first_half = &s[..mid];
        let second_half = &s[mid..];
        if first_half == second_half{
            return false;
        }
    }
    return true;
}

pub fn is_valid_id_2(n:i64) -> bool {
    // Id is invalid if it is made only of some sequence of digits repeated at least twice.
    // turn to string
    let s = n.to_string();
    let len = s.len();
    // try every possible pattern length up to half the string length
    for l in 1..=len/2 {
        if len % l != 0 {
            continue;
        }
        let pattern = &s[..l];
        if pattern.repeat(len / l) == s {
            return false;
        }
    }
    return true;
}

pub fn part_one(input: &str) -> Option<i64> {
    let ranges = input.split(',');
    let mut sum = 0;
    for range in ranges {
        let spl = range.split('-').collect::<Vec<&str>>();
        let start = spl[0].parse::<i64>().unwrap();
        let end = spl[1].parse::<i64>().unwrap();
        for n in start..=end {
            if !is_valid_id(n) {
                sum += n;
            }
        }
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<i64> {
    let ranges = input.split(',');
    let mut sum = 0;
    for range in ranges {
        let spl = range.split('-').collect::<Vec<&str>>();
        let start = spl[0].parse::<i64>().unwrap();
        let end = spl[1].parse::<i64>().unwrap();
        for n in start..=end {
            if !is_valid_id_2(n) {
                sum += n;
            }
        }
    }
    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_example_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(16793817782));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(27469417404));
    }
}
