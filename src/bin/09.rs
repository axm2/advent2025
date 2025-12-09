use std::collections::HashSet;
advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut max_area = 0;
    let coords: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x: usize = parts.next().unwrap().parse().unwrap();
            let y: usize = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();
    for coord in coords.iter() {
        for other in coords.iter() {
            let area = (coord.0.abs_diff(other.0) + 1) * (coord.1.abs_diff(other.1) + 1);
            if area > max_area {
                max_area = area;
            }
        }
    }
    Some(max_area as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Build the perimeter
    // Then calculate the max areas like we did in step one, but keep multiple.
    // For each max area, raytrace and confirm all points are within the boundaries.
    // Because the max areas are already sorted, we can just return the first one that works.
    let coords: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x: usize = parts.next().unwrap().parse().unwrap();
            let y: usize = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();
    let mut perimeter: HashSet<(usize, usize)> = HashSet::from_iter(coords);
    for i in 0..input.len() {
        let prev;
        let next;
        if i == 0 {
            prev = coords.last().unwrap();
        } else {
            prev = coords.iter().nth(i - 1)?;
        }
        if i == coords.len() - 1 {
            next = coords.first().unwrap();
        } else {
            next = coords.iter().nth(i+1));
        }
        // Insert line between coords[i] and prev and line between coords[i] and next
        let (x, y) = coords[i];
        let (px, py) = *prev;
        let (nx, ny) = *next;
        if prev.0 == x {
            // Vertical line
            let range = if py < y { py..=y } else { y..=py };
            for yy in range {
                perimeter.insert((x, yy));
            }
        } else if prev.1 == y {
            // Horizontal line
            let range = if px < x { px..=x } else { x..=px };
            for xx in range {
                perimeter.insert((xx, y));
            }
        }
        if next.0 == x {
            // Vertical line
            let range = if ny < y { ny..=y } else { y..=ny };
            for yy in range {
                perimeter.insert((x, yy));
            }
        } else if next.1 == y {
            // Horizontal line
            let range = if nx < x { nx..=x } else { x..=nx };
            for xx in range {
                perimeter.insert((xx, y));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_example_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
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
