use std::fs;

pub fn run() {
        let universe = fs::read_to_string("11.txt").unwrap();
        println!("{universe}\n\n");
        let transposed_universe = transpose_universe(&universe);

        let expanded_universe = expand_universe(&transposed_universe); 

        let expanded_universe = transpose_universe(&expanded_universe);
        let expanded_universe = transpose_universe(&expanded_universe);
        let expanded_universe = transpose_universe(&expanded_universe);

        println!("{expanded_universe}");
}

fn transpose_universe(universe: &str) -> String {
    let universe: Vec<_> = universe.lines().map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect();

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
