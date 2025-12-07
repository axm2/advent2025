use std::collections::HashMap;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u128> {
    let mut result = 0u128;
    let mut map: HashMap<usize, Vec<u128>> = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();
    for line in &lines[..lines.len() - 1] {
        for (idx, num) in line.split_whitespace().enumerate() {
            map.entry(idx as usize)
                .and_modify(|v| v.push(num.parse::<u128>().unwrap()))
                .or_insert(vec![num.parse::<u128>().unwrap()]);
        }
    }
    for (idx, operator) in lines.last()?.split_whitespace().enumerate() {
        if operator == "+" {
            result += map.get(&(idx as usize)).unwrap().iter().sum::<u128>();
        } else if operator == "*" {
            result += map.get(&(idx as usize)).unwrap().iter().product::<u128>();
        }
    }
    return Some(result);
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0u64;
    let mut map: HashMap<(usize, usize), char> = HashMap::new();
    for (line, row) in input.lines().enumerate() {
        if line != input.lines().count() - 1 {
            for (ch, col) in row.chars().enumerate() {
                map.insert((line, ch), col);
            }
        }
    }
    let mut count = 0;
    let mut current_operator = ' ';
    for k in 0..input.lines().last()?.chars().count() {
        let char = input.lines().last()?.chars().nth(k).unwrap();
        if k == 0 {
            current_operator = char;
            continue;
        }
        if !vec!['+', '*'].contains(&char) && k != input.lines().last()?.chars().count() - 1 {
            count += 1;
        } else if vec!['+', '*'].contains(&char) || k == input.lines().last()?.chars().count() - 1 {
            // we collect the digits for the previous operator
            let mut operands: Vec<u64> = Vec::new();
            let mut digits: Vec<char> = Vec::new();
            // for j in (k..count+k).rev(){
            let low: usize;
            let high: usize;
            if k == input.lines().last()?.chars().count() - 1 {
                low = k - count - 1;
                high = k + 1;
            } else {
                low = k - 1 - count;
                high = k - 1;
            }
            for j in low..high {
                for i in 0..input.lines().count() {
                    let digit = map.get(&(i as usize, j as usize));
                    if digit.is_none() {
                        continue;
                    }
                    if digit.unwrap().is_digit(10) {
                        digits.push(*digit.unwrap());
                    }
                }
                // take the digits, add it to a vector
                // convert digits to number
                let number_str: String = digits.iter().collect();
                let number: u64 = number_str.parse().unwrap();
                operands.push(number);
                digits.clear();
            }
            // println!("{:?}", operands);
            if current_operator == '+' {
                let subtotal = operands.iter().sum::<u64>();
                total += subtotal;
                // println!("Subtotal for +: {}", subtotal);
            } else if current_operator == '*' {
                let subtotal = operands.iter().product::<u64>();
                total += subtotal;
                // println!("Subtotal for *: {}", subtotal);
            }
            // reset count
            count = 0;
            current_operator = char;
        }
        // println!("Current operator: {}", current_operator);
    }
    return Some(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_example_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
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
