advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for bank in input.lines(){
        let mut h_1 = 0;
        let mut h_2 = 0;
        for n in bank.chars().filter_map(|c| c.to_digit(10)){
            if h_1 < h_2 {
                std::mem::swap(&mut h_1, &mut h_2);
            }
            if n > h_2 {
                h_2 = n;
            }
        }
        let intermediate = h_1*10+h_2;
        total+=intermediate;
    }
    return Some(total);
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY,0));
        assert_eq!(result, Some(381));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
