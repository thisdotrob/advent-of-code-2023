use std::fs;

pub fn run() {
    let universe = fs::read_to_string("11.txt").unwrap();
    println!("pt1: {}", pt1(&universe));
}

fn pt1(universe: &str) -> usize {
    let transposed_universe = transpose_universe(&universe);

    let expanded_universe = expand_universe(&transposed_universe);

    // this is stupid but I went with the approach because I wanted to figure out how to transpose a Vec<Vec<_>>
    let expanded_universe = transpose_universe(&expanded_universe);
    let expanded_universe = transpose_universe(&expanded_universe);
    let expanded_universe = transpose_universe(&expanded_universe);

    let expanded_universe = expand_universe(&expanded_universe);

    let lengths = shortest_path_lengths(&expanded_universe);

    lengths.iter().sum()
}

fn transpose_universe(universe: &str) -> String {
    let universe: Vec<_> = universe
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let universe_height = universe.len();
    let universe_width = universe[0].len();

    let mut transposed = vec![];

    for i in 0..universe_width {
        let mut col = vec![];
        for j in 0..universe_height {
            let row = &universe[j];
            let spot = row[i];
            col.push(spot);
        }
        let col = col.iter();
        let col = col.collect::<String>();
        transposed.push(col);
    }

    transposed.join("\n")
}

fn expand_universe(universe: &str) -> String {
    let mut expanded_universe = vec![];

    let first_line = universe.lines().next().unwrap();
    let line_length = first_line.len();
    let blank_line = (0..line_length).map(|_| ".").collect::<String>();

    for line in universe.lines() {
        expanded_universe.push(line);
        if line.chars().all(|point| point == '.') {
            expanded_universe.push(&blank_line);
        }
    }

    expanded_universe.join("\n")
}

fn galaxy_pairs(mut galaxy_positions: Vec<[usize; 2]>) -> Vec<[[usize; 2]; 2]> {
    let mut pairs: Vec<[[usize; 2]; 2]> = vec![];
    let mut head = galaxy_positions.pop();

    while let Some(pos_a) = head {
        for pos_b in &galaxy_positions {
            pairs.push([pos_a, *pos_b]);
        }
        head = galaxy_positions.pop();
    }
    pairs
}

fn shortest_path_lengths(universe: &str) -> Vec<usize> {
    let positions = galaxy_positions(universe);
    let pairs = galaxy_pairs(positions);
    let mut path_lengths = vec![];
    for [pos_1, pos_2] in pairs {
        // Assumes pairs are ordered with galaxy nearest start of universe first
        let vertical_distance = pos_1[0].abs_diff(pos_2[0]);
        let horizontal_distance = pos_1[1].abs_diff(pos_2[1]);
        let total_distance = vertical_distance + horizontal_distance;
        path_lengths.push(total_distance);
    }
    path_lengths
}

fn galaxy_positions(universe: &str) -> Vec<[usize; 2]> {
    let mut positions = vec![];
    let mut i = 0;
    let mut j = 0;
    for char in universe.chars() {
        if char == '#' {
            positions.push([i, j]);
            j += 1;
        } else if char == '\n' {
            i += 1;
            j = 0;
        } else {
            j += 1;
        }
    }
    positions
}

#[cfg(test)]
mod pt1_tests {
    use super::*;

    #[test]
    fn example_input() {
        let universe = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(374, pt1(universe));
    }
}

#[cfg(test)]
mod galaxy_positions_tests {
    use super::*;

    #[test]
    fn two_galaxies() {
        let universe = "#..
...
..#";
        assert_eq!(vec![[0, 0], [2, 2]], galaxy_positions(universe));
    }

    #[test]
    fn example_input_truncated() {
        let universe = ".........#...
#....#.......";

        let expected = vec![[0, 9], [1, 0], [1, 5]];

        assert_eq!(expected, galaxy_positions(universe));
    }

    #[test]
    fn example_input() {
        let universe = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";

        let expected = vec![
            [0, 4],
            [1, 9],
            [2, 0],
            [5, 8],
            [6, 1],
            [7, 12],
            [10, 9],
            [11, 0],
            [11, 5],
        ];

        assert_eq!(expected, galaxy_positions(universe));
    }
}

#[cfg(test)]
mod shortest_path_lengths_tests {
    use super::*;

    #[test]
    fn two_galaxies() {
        let universe = "#..
...
..#";

        let expected = vec![4];
        assert_eq!(expected, shortest_path_lengths(universe));
    }

    #[test]
    fn two_galaxies_negative_difference_in_horizontal_distance() {
        let universe = "..#
...
#..";
        let expected = vec![4];
        assert_eq!(expected, shortest_path_lengths(universe));
    }

    #[test]
    fn galaxies_5_and_9_from_example_input() {
        let universe = ".............
.............
.............
.............
.............
.............
.#...........
.............
.............
.............
.............
.....#.......";

        let expected = vec![9];
        assert_eq!(expected, shortest_path_lengths(universe));
    }

    #[test]
    fn galaxies_1_and_7_from_example_input() {
        let universe = "....#........
.............
.............
.............
.............
.............
.............
.............
.............
.............
.........#...
.............";

        let expected = vec![15];
        assert_eq!(expected, shortest_path_lengths(universe));
    }

    #[test]
    fn galaxies_3_and_6_from_example_input() {
        let universe = ".............
.............
#............
.............
.............
.............
.............
............#
.............
.............
.............
.............";

        let expected = vec![17];

        assert_eq!(expected, shortest_path_lengths(universe));
    }

    #[test]
    fn galaxies_8_and_9_from_example_input() {
        let universe = ".............
.............
.............
.............
.............
.............
.............
.............
.............
.............
.............
#....#.......";

        let expected = vec![5];

        assert_eq!(expected, shortest_path_lengths(universe));
    }
}

#[cfg(test)]
mod transpose_universe_tests {
    use super::*;

    #[test]
    fn transposes_the_universe_anticlockwise() {
        let universe = "....
####
....
####";

        let expected = ".#.#
.#.#
.#.#
.#.#";

        assert_eq!(expected, transpose_universe(universe));
    }
}

#[cfg(test)]
mod expand_universe_tests {
    use super::*;

    #[test]
    fn expands_empty_rows() {
        let universe = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let expected = "...#......
.......#..
#.........
..........
..........
......#...
.#........
.........#
..........
..........
.......#..
#...#.....";

        assert_eq!(expected, expand_universe(&universe));
    }

    #[test]
    fn expands_empty_columns_when_used_with_transpose() {
        let universe = "#.#
#.#
#.#";
        let expected = "#..#
#..#
#..#";
        let transposed_universe = transpose_universe(universe);

        let expanded_universe = expand_universe(&transposed_universe);

        let expanded_universe = transpose_universe(&expanded_universe);
        let expanded_universe = transpose_universe(&expanded_universe);
        let expanded_universe = transpose_universe(&expanded_universe);

        assert_eq!(expected, expanded_universe);
    }
}
