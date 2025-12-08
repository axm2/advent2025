use std::collections::{HashMap, HashSet};

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

pub fn dfs(
    grid: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    memo: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    let height = grid.len();
    let width = grid[0].len();

    // Past the bottom of the grid means this timeline exits successfully.
    if i == height {
        return 1;
    }

    // Out-of-bounds horizontally means this branch leaves the manifold.
    if j >= width {
        return 0;
    }

    if memo.contains_key(&(i, j)) {
        return *memo.get(&(i, j)).unwrap();
    }
    let ch = grid[i][j];
    let mut total_paths = 0u64;

    if ch == '^' {
        // Split into left and right branches on the next row, guarding bounds.
        if j > 0 {
            total_paths += dfs(grid, i + 1, j - 1, memo);
        }
        if j + 1 < width {
            total_paths += dfs(grid, i + 1, j + 1, memo);
        }
    } else {
        // Continue straight down for empty space or the start cell.
        total_paths += dfs(grid, i + 1, j, memo);
    }

    memo.insert((i, j), total_paths);
    total_paths
}

pub fn part_two(input: &str) -> Option<u64> {
    // depth first traversal with memoization
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut memo: HashMap<(usize, usize), u64> = HashMap::new();
    let mut timelines = 0u64;
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if *ch == 'S' {
                timelines = dfs(&grid, i, j, &mut memo);
            }
        }
    }
    Some(timelines)
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
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
