#![feature(int_roundings)]
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut dial = 50;
    let mut count = 0;
    for _line in input.lines() {
        if _line.starts_with('R') {
            dial += _line[1..].parse::<i32>().unwrap();
        } else if _line.starts_with('L') {
            dial -= _line[1..].parse::<i32>().unwrap();
        }
        if dial % 100 == 0 {
            count += 1;
        }
    }
    return Some(count);
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut dial = 50;
    let mut count = 0;
    for _line in input.lines() {
        if _line.starts_with('R') {
            dial += _line[1..].parse::<i32>().unwrap();
            if dial == 0 {
                count += 1;
            }
            count += dial.div_floor(100);
            dial = dial.rem_euclid(100);
        }
        // we were at 0 and turned left small - no
        // we were at 0 and turned left big - y if dial is below -100
        // we were positive and turned to zero - yes
        // we were positive and turned negative - yes
        else if _line.starts_with('L') {
            let old_dial = dial;
            dial -= _line[1..].parse::<i32>().unwrap();
            if old_dial > 0 && dial <= 0 {
                // how many times did we cross 0?
                count += 1 + (dial).div_floor(-100);
            }
            if old_dial == 0 && dial < -100 {
                count += (dial).div_floor(-100);
            }
            dial = dial.rem_euclid(100);
        }
    }
    return Some(count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_example_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
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
