advent_of_code::solution!(4);

pub fn is_accessible(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    // accessible if fewer than 4 neighbors are '@'
    let mut roll_count = 0;
    // 8 directions as isize deltas
    let directions: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let rows = matrix.len() as isize;
    let cols = if rows > 0 {
        matrix[0].len() as isize
    } else {
        0
    };
    let i_isize = i as isize;
    let j_isize = j as isize;

    for (di, dj) in directions.iter() {
        let ni = i_isize + di;
        let nj = j_isize + dj;
        if ni >= 0 && ni < rows && nj >= 0 && nj < cols {
            if matrix[ni as usize][nj as usize] == '@' {
                roll_count += 1;
            }
        }
    }

    // accessible if fewer than 4 neighboring '@'
    roll_count < 4
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut accessible = 0usize;
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '@' && is_accessible(&matrix, i, j) {
                accessible += 1;
            }
        }
    }
    Some(accessible as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut removed_count = 0usize;
    let mut flag: bool = false;
    while !flag {
        let mut removed: Vec<(i32, i32)> = Vec::new();
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == '@' && is_accessible(&matrix, i, j) {
                    removed.push((i as i32, j as i32));
                    removed_count += 1;
                }
            }
        }
        if removed.len() == 0 {
            flag = true;
        } else {
            for (i, j) in removed {
                matrix[i as usize][j as usize] = '.';
            }
        }
    }
    Some(removed_count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_example_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
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
