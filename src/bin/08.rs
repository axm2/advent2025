#![feature(binary_heap_into_iter_sorted)]
use std::fmt;
advent_of_code::solution!(8);

struct PointPair {
    point_a: Point,
    point_b: Point,
    distance: u64,
}
impl Ord for PointPair {
    // BinaryHeap is a max-heap, we want a min-heap
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}
impl PartialOrd for PointPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for PointPair {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
            && self.point_a == other.point_a
            && self.point_b == other.point_b
    }
}
impl Eq for PointPair {}

#[derive(Copy, Clone)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}
impl Point {
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x as f64 - other.x as f64;
        let dy = self.y as f64 - other.y as f64;
        let dz = self.z as f64 - other.z as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // O(n^2)
    let mut heap = std::collections::BinaryHeap::new();
    for line in input.lines() {
        let mut parts = line.split(',');
        let current_point: Point = Point {
            x: parts.next()?.parse::<u64>().ok()?,
            y: parts.next()?.parse::<u64>().ok()?,
            z: parts.next()?.parse::<u64>().ok()?,
        };
        let mut closest_point: PointPair = PointPair {
            point_a: current_point,
            point_b: Point { x: 0, y: 0, z: 0 },
            distance: u64::MAX,
        };

        for line in input.lines() {
            let mut parts = line.split(',');
            let new_point: Point = Point {
                x: parts.next()?.parse::<u64>().ok()?,
                y: parts.next()?.parse::<u64>().ok()?,
                z: parts.next()?.parse::<u64>().ok()?,
            };
            if new_point != current_point {
                let dist = current_point.distance(&new_point);
                if dist < closest_point.distance as f64 {
                    // update closest point
                    closest_point = PointPair {
                        point_a: current_point,
                        point_b: new_point,
                        distance: dist as u64,
                    };
                }
            }
        }
        heap.push(closest_point);
    }
    println!(
        "Pop irst five from heap: {}",
        heap.into_iter_sorted()
            .take(7)
            .map(|pd| format!("{} - {}: {}", pd.point_a, pd.point_b, pd.distance))
            .collect::<Vec<String>>()
            .join(", ")
    );
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
