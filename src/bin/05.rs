advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i64> {
    let (ranges, ingredient_ids) = input.split_once("\n\n")?;
    let mut sum = 0i64;
    for ingredient_id in ingredient_ids.lines() {
        for range in ranges.lines() {
            if ingredient_id.parse::<i64>().unwrap()
                >= range.split('-').collect::<Vec<&str>>()[0]
                    .parse::<i64>()
                    .unwrap()
                && ingredient_id.parse::<i64>().unwrap()
                    <= range.split('-').collect::<Vec<&str>>()[1]
                        .parse::<i64>()
                        .unwrap()
            {
                // fresh
                sum += 1;
                break;
            } else {
                //spoiled
            }
        }
    }
    return Some(sum);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0u64;
    let (ranges, _) = input.split_once("\n\n")?;
    let mut range_list: Vec<(i64, i64)> = Vec::new();

    for range in ranges.lines() {
        // parse once and make mutable
        let (mut current_start, mut current_end) = match range.split_once('-') {
            Some((s, e)) => (s.trim().parse::<i64>().ok()?, e.trim().parse::<i64>().ok()?),
            None => continue,
        };

        // iterate by index so we can mutate the vector safely
        let mut i = 0usize;
        while i < range_list.len() {
            let existing = range_list[i];

            // no overlap
            if current_end < existing.0 || current_start > existing.1 {
                i += 1;
                continue;
            }

            // current fully covers existing -> remove existing
            if current_start < existing.0 && current_end > existing.1 {
                range_list.swap_remove(i);
                // don't increment i; new element at i must be examined
                continue;
            }

            // existing fully covers current -> discard current
            if current_start >= existing.0 && current_end <= existing.1 {
                current_start = 1;
                current_end = 0;
                break;
            }

            // overlap at the low end of existing: trim end of current
            if current_start < existing.0 && current_end >= existing.0 && current_end <= existing.1
            {
                current_end = existing.0 - 1;
                if current_start > current_end {
                    break;
                }
                i += 1;
                continue;
            }

            // overlap at the high end of existing: trim start of current
            if current_start >= existing.0
                && current_start <= existing.1
                && current_end > existing.1
            {
                current_start = existing.1 + 1;
                if current_start > current_end {
                    break;
                }
                i += 1;
                continue;
            }

            // any other overlap case we didn't explicitly catch: advance
            i += 1;
        }

        // if there's any remaining (non-empty) part of current, add it
        if current_start <= current_end {
            range_list.push((current_start, current_end));
        }
    }

    // sum lengths
    for (s, e) in &range_list {
        total += (*e - *s + 1) as u64;
    }

    Some(total)
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
        assert_eq!(result, Some(14));
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
