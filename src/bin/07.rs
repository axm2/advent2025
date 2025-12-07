use std::collections::HashSet;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    // S starts at the top, goes down.
    // For the first row we find S. and note that down as a vertical beam.
    // For each subsequent row we check if there's a splitter at the same vertical index as our downward beam.
    // If there is, we add it to our list.
    // Continue until we reach the bottom then return the number of beams.
    let mut beams: HashSet<usize> = HashSet::new();
    let mut counter = 0u64;
    for (i, line) in input.lines().enumerate() {
        // if i % 2 == 1 {
        //     println!("Beams: {:?}", beams.len());
        // }
        if i == input.lines().count() - 1 {
            // return Some(beams.len() as u64);
            return Some(counter);
        } else {
            for (j, ch) in line.chars().enumerate() {
                if i == 0 && ch == 'S' {
                    beams.insert(j);
                } else if ch == '^' && beams.contains(&j) {
                    beams.remove(&j);
                    beams.insert(j + 1);
                    beams.insert(j - 1);
                    counter += 1;
                }
            }
        }
    }
    return Some(0u64);
}

pub fn part_two(input: &str) -> Option<u64> {
    // Make a hashmap, the key will be a list of each split choice.
    // When we hit a split, we create a new array with the previous choices plus the new choice, and push it onto a stack.
    // At the end of all choices, we add the key to the hashmap, then we move on to the next in the stack.
    // We return the length of the hashmap at the end (unique paths).
    //     let mut paths: HashSet<String> = HashSet::new();
    //     let mut stack: Vec<(usize, String)> = Vec::new();
    //     for (i, line) in input.lines().enumerate() {
    //         if i == 0 {
    //             for (j, ch) in line.chars().enumerate() {
    //                 if ch == 'S' {
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_example_part_two() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
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
