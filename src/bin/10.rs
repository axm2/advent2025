advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let re = regex::new(
        r"(?<diagram>\[.*\])[[:space:]]+(?<schematics>\(.*\))[[:space:]](?<joltages>.*)",
    )
    .unwrap();
    for line in input.lines() {
        // Given a line like [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        // We want to split into three groups
        // (everything in the brackets,
        // the groups in parens,
        // and the numbers in the curly braces)
        let caps = re.captures(line).unwrap();
        let diagram = &caps["diagram"];
        let schematics = &caps["schematics"];
        let _joltages = &caps["joltages"];

        // Solution can be represented as binary, 0110
        // Buttons can be represented as functions...
        // (3) = 0001
        // (1,3) = 0101
        // (2) = 0010
        // (2,3) = 0011
        // Then they all get added but with an exponent. So the equation will be
        // 0110 = 1010 + 1100 because 1plus1 cancels out and we can just truncate.
        //
        // So algorithmically, how do we solve this system of equations? RREF?
    }
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_example_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    #[ignore]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, None);
    }
}
