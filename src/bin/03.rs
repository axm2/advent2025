advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for bank in input.lines(){
        // let mut v = vec![0u32; 2];
        let mut a = [0u32; 2];
        for n in bank.chars().filter_map(|c| c.to_digit(10)){
            if a[0]*10 + a[1] < a[1]*10 + n {
                a[0] = a[1];
                a[1] = n;
            } else if a[1]<n{
                a[1] = n;
            }
        }
        let intermediate= a[0]*10+a[1];
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(17092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
