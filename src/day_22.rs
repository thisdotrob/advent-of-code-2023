use std::cmp::{max, min, Ordering};
use std::collections::{HashMap, HashSet};
use std::fmt;

use std::fs;

pub fn run() {
    let input = fs::read_to_string("22_example.txt").unwrap();
    println!("pt1 example: {}", pt1(&input));
    let input = fs::read_to_string("22.txt").unwrap();
    println!("pt1: {}", pt1(&input));
}

fn pt1(input: &str) -> usize {
    let bricks_by_z = parse_input(input);

    let bricks_by_z = apply_gravity(bricks_by_z);

    let mut answer = 0;

    let mut supported_bricks: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut supporting_bricks: HashMap<usize, HashSet<usize>> = HashMap::new();

    for brick in bricks_by_z.iter().flatten() {
        let min_z = brick.min_z();
        let max_z = brick.max_z();
        let supported_bricks_entry = supported_bricks.entry(brick.2).or_insert(HashSet::new());
        let supporting_bricks_entry = supporting_bricks.entry(brick.2).or_insert(HashSet::new());
        if max_z < bricks_by_z.len() - 1 {
            for brick_above in bricks_by_z[max_z + 1..bricks_by_z.len()].iter().flatten() {
                if brick_above.is_supported_by(brick) {
                    supported_bricks_entry.insert(brick_above.2);
                }
            }
        }
        if min_z > 1 {
            for brick_below in bricks_by_z[1..min_z].iter().flatten() {
                if brick.is_supported_by(brick_below) {
                    supporting_bricks_entry.insert(brick_below.2);
                }
            }
        }
    }

    for brick in bricks_by_z.iter().flatten() {
        let supported_bricks_entry = supported_bricks.get(&brick.2).unwrap();
        let mut can_disintegrate = true;
        for supported_brick in supported_bricks_entry {
            let supporting_bricks_entry = supporting_bricks.get(&supported_brick).unwrap();
            if supporting_bricks_entry.len() == 1 {
                can_disintegrate = false;
            }
        }
        if can_disintegrate {
            answer += 1;
        }
    }

    answer
}

fn parse_input(input: &str) -> Vec<Vec<Brick>> {
    let mut bricks: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (start_coord, end_coord) = line.split_once("~").unwrap();

            let start_coord: Vec<usize> =
                start_coord.split(",").map(|s| s.parse().unwrap()).collect();
            let end_coord: Vec<usize> = end_coord.split(",").map(|s| s.parse().unwrap()).collect();

            Brick(
                (start_coord[0], start_coord[1], start_coord[2]),
                (end_coord[0], end_coord[1], end_coord[2]),
                i + 1,
            )
        })
        .collect();

    bricks.sort_unstable();

    let mut bricks_by_z = vec![vec![]];

    let mut bricks_at_z: Vec<Brick> = Vec::new();

    for brick in bricks {
        let max_z = max(brick.0 .2, brick.1 .2);
        while bricks_by_z.len() - 1 != max_z - 1 {
            bricks_by_z.push(std::mem::take(&mut bricks_at_z));
        }
        bricks_at_z.push(brick);
    }

    bricks_by_z.push(bricks_at_z);

    bricks_by_z
}

fn apply_gravity(mut bricks_by_z: Vec<Vec<Brick>>) -> Vec<Vec<Brick>> {
    let mut falling_bricks = vec![];

    loop {
        for z in 2..bricks_by_z.len() {
            for i in 0..bricks_by_z[z].len() {
                let mut is_supported = false;
                let brick = &bricks_by_z[z][i];
                let min_z = brick.min_z();

                if min_z == 1 {
                    continue;
                }

                for ii in 0..bricks_by_z[min_z - 1].len() {
                    let other_brick = &bricks_by_z[min_z - 1][ii];
                    if bricks_by_z[z][i].is_supported_by(other_brick) {
                        is_supported = true;
                    }
                }
                if !is_supported {
                    falling_bricks.push((z, i));
                }
            }
        }

        if falling_bricks.is_empty() {
            break;
        } else {
            while !falling_bricks.is_empty() {
                let (z, i) = falling_bricks.pop().unwrap();
                let mut brick = bricks_by_z[z].remove(i);
                brick.fall();
                bricks_by_z[z - 1].push(brick);
            }
        }
    }

    bricks_by_z
}

#[derive(Eq, PartialEq)]
struct Brick((usize, usize, usize), (usize, usize, usize), usize);

impl Brick {
    fn fall(&mut self) {
        self.0 .2 = self.0 .2 - 1;
        self.1 .2 = self.1 .2 - 1;
    }

    fn is_supported_by(&self, other: &Self) -> bool {
        self.min_z() == other.max_z() + 1
            && self.min_x() <= other.max_x()
            && other.min_x() <= self.max_x()
            && self.min_y() <= other.max_y()
            && other.min_y() <= self.max_y()
    }

    fn max_z(&self) -> usize {
        max(self.0 .2, self.1 .2)
    }

    fn min_z(&self) -> usize {
        min(self.0 .2, self.1 .2)
    }

    fn max_y(&self) -> usize {
        max(self.0 .1, self.1 .1)
    }

    fn max_x(&self) -> usize {
        max(self.0 .0, self.1 .0)
    }

    fn min_y(&self) -> usize {
        min(self.0 .1, self.1 .1)
    }

    fn min_x(&self) -> usize {
        min(self.0 .0, self.1 .0)
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_max_z = max(self.0 .2, self.1 .2);
        let other_max_z = max(other.0 .2, other.1 .2);
        self_max_z.cmp(&other_max_z)
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for Brick {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "Brick {}: {},{},{}~{},{},{}",
            self.2, self.0 .0, self.0 .1, self.0 .2, self.1 .0, self.1 .1, self.1 .2
        )
    }
}

#[cfg(test)]
mod day_22_pt_1_tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "1,0,3~1,0,1
1,0,4~1,0,7";

        let bricks_by_z = parse_input(input);

        assert_eq!(0, bricks_by_z[0].len());
        assert_eq!(0, bricks_by_z[1].len());
        assert_eq!(0, bricks_by_z[2].len());
        assert_eq!(1, bricks_by_z[3].len());
        assert_eq!(0, bricks_by_z[4].len());
        assert_eq!(0, bricks_by_z[5].len());
        assert_eq!(0, bricks_by_z[6].len());
        assert_eq!(1, bricks_by_z[7].len());
    }

    #[test]
    fn test_apply_gravity_with_no_falling_bricks() {
        let input = "1,0,3~1,0,1
1,0,4~1,0,7";

        let bricks_by_z = parse_input(input);

        let bricks_by_z = apply_gravity(bricks_by_z);

        assert_eq!(0, bricks_by_z[0].len());
        assert_eq!(0, bricks_by_z[1].len());
        assert_eq!(0, bricks_by_z[2].len());
        assert_eq!(1, bricks_by_z[3].len());
        assert_eq!(1, bricks_by_z[3][0].2);
        assert_eq!(0, bricks_by_z[4].len());
        assert_eq!(0, bricks_by_z[5].len());
        assert_eq!(0, bricks_by_z[6].len());
        assert_eq!(1, bricks_by_z[7].len());
        assert_eq!(2, bricks_by_z[7][0].2);
    }

    #[test]
    fn test_apply_gravity() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

        let bricks_by_z = parse_input(input);

        let bricks_by_z = apply_gravity(bricks_by_z);

        assert_eq!(0, bricks_by_z[0].len());
        assert_eq!(1, bricks_by_z[1].len());
        assert_eq!(1, bricks_by_z[1][0].2);
        assert_eq!(2, bricks_by_z[2].len());
        assert_eq!(2, bricks_by_z[2][0].2);
        assert_eq!(3, bricks_by_z[2][1].2);
        assert_eq!(2, bricks_by_z[3].len());
        assert_eq!(4, bricks_by_z[3][0].2);
        assert_eq!(5, bricks_by_z[3][1].2);
        assert_eq!(1, bricks_by_z[4].len());
        assert_eq!(6, bricks_by_z[4][0].2);
        assert_eq!(0, bricks_by_z[5].len());
        assert_eq!(1, bricks_by_z[6].len());
        assert_eq!(7, bricks_by_z[6][0].2);
    }
}
