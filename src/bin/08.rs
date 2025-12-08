#![feature(binary_heap_into_iter_sorted)]
use std::{fmt, hash::Hash};
use std::collections::HashSet;
advent_of_code::solution!(8);

struct PointPair {
    point_a: Point,
    point_b: Point,
    distance: f64,
}
impl Ord for PointPair {
    // BinaryHeap is a max-heap, we want a min-heap
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.total_cmp(&self.distance)
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
impl Eq for Point {}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
impl Hash for Point {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.z.hash(state);
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // O(n^2)
    let mut heap = std::collections::BinaryHeap::new();
    let points: Vec<Point> = input.lines().filter_map(|line| {
        let mut parts = line.split(',');
        Some(Point {
            x: parts.next()?.parse::<u64>().ok()?,
            y: parts.next()?.parse::<u64>().ok()?,
            z: parts.next()?.parse::<u64>().ok()?,
        })
    }).collect();
    for current_point in &points {
        let mut closest_point: PointPair = PointPair {
            point_a: *current_point,
            point_b: Point { x: 0, y: 0, z: 0 },
            distance: f64::MAX,
        };
        for new_point in &points {
            if *new_point != *current_point {
                let dist = current_point.distance(new_point);
                if dist < closest_point.distance as f64 {
                    // update closest point
                    closest_point = PointPair {
                        point_a: *current_point,
                        point_b: *new_point,
                        distance: dist,
                    };
                }
            }
        }
        heap.push(closest_point);
    }
    // Initialize a vector of hashsets. Each hashset will represent a cluster of points.
    // Pop one off the heap, if neither point is in a cluster, create a new cluster with those two points.
    // If one point is in a cluster, we add the other point to that cluster.
    let mut circuits: Vec<HashSet<Point>> = Vec::from_iter(points.iter().map(|p| {
        let mut hs = HashSet::new();
        hs.insert(*p);
        hs
    }));
    for _ in 0..1000{
        if let Some(pp) = heap.pop() {
            let mut cluster_a_index: Option<usize> = None;
            let mut cluster_b_index: Option<usize> = None;
            for (i, cluster) in circuits.iter().enumerate() {
                if cluster.contains(&pp.point_a) {
                    cluster_a_index = Some(i);
                }
                if cluster.contains(&pp.point_b) {
                    cluster_b_index = Some(i);
                }
            }
            match (cluster_a_index, cluster_b_index) {
                (Some(a), Some(b)) if a != b => {
                    // merge clusters
                    let cluster_b = circuits.remove(b);
                    for point in cluster_b {
                        circuits[a].insert(point);
                    }
                }
                (Some(a), None) => {
                    circuits[a].insert(pp.point_b);
                }
                (None, Some(b)) => {
                    circuits[b].insert(pp.point_a);
                }
                (None, None) => {
                    let mut new_cluster = HashSet::new();
                    new_cluster.insert(pp.point_a);
                    new_cluster.insert(pp.point_b);
                    circuits.push(new_cluster);
                }
                _ => {}
            }
        }
    }
    println!(
        "Circuits: {}",
        circuits
            .iter()
            .map(|c| c.len().to_string())
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
        assert_eq!(result, Some(40));
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
