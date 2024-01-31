use std::collections::{VecDeque, HashSet};

use std::fs;

pub fn run() {
    let input = fs::read_to_string("23_example.txt").unwrap();
    println!("pt1 example: {}", pt1(&input));
}

fn pt1(input: &str) -> usize {
    let map = parse_input(input);
    let start_position = find_start_position(&map);
    let end_position = find_end_position(&map);

    let mut routes: VecDeque<Route> = VecDeque::new();

    let route = Route {
        tiles_visited: vec![start_position],
        current_tile: start_position,
    };

    routes.push_back(route);

    while let Some(route) = routes.pop_front() {
        neighbours(route.current_tile, &map);
    }

    0
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_start_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    let y = 0;
    let mut x = 0;
    let mut row = map[y].iter();
    while let '#' = row.next().unwrap() {
        x += 1
    }
    (x, y)
}

fn find_end_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    let y = map.len() - 1;
    let mut x = 0;
    let mut row = map[y].iter();
    while let '#' = row.next().unwrap() {
        x += 1
    }
    (x, y)
}

fn neighbours(position: (usize, usize), map: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut neighbours = HashSet::new();

    let (x, y) = position;

    if x > 0 {
        neighbours.insert((x - 1, y));
    }

    if y > 0 {
        neighbours.insert((x, y - 1));
    }

    if x < (map[0].len() - 1) {
        neighbours.insert((x + 1, y));
    }

    if y < (map.len() - 1) {
        neighbours.insert((x, y + 1));
    }

    neighbours
}

struct Route {
    tiles_visited: Vec<(usize, usize)>,
    current_tile: (usize, usize)
}

#[cfg(test)]
mod day_23_pt_1_tests {
    use super::*;

    #[test]
    fn test_find_start_position() {
        let input = fs::read_to_string("23_example.txt").unwrap();
        let map = parse_input(&input);
        let start_position = find_start_position(&map);
        assert_eq!((1, 0), start_position);
    }

    #[test]
    fn test_find_end_position() {
        let input = fs::read_to_string("23_example.txt").unwrap();
        let map = parse_input(&input);
        let end_position = find_end_position(&map);
        assert_eq!((21, 22), end_position);
    }

    #[test]
    fn test_neighbours_top_left_corner() {
        let input = fs::read_to_string("23_example.txt").unwrap();
        let map = parse_input(&input);
        let position = (0,0);
        let expected_neighbours = HashSet::from([(0,1), (1,0)]);
        assert_eq!(expected_neighbours, neighbours(position, &map));
    }

    #[test]
    fn test_neighbours_top_right_corner() {
        let input = fs::read_to_string("23_example.txt").unwrap();
        let map = parse_input(&input);
        let position = (22,0);
        let expected_neighbours = HashSet::from([(21,0), (22,1)]);
        assert_eq!(expected_neighbours, neighbours(position, &map));
    }

    #[test]
    fn test_neighbours_bottom_left_corner() {
        let input = fs::read_to_string("23_example.txt").unwrap();
        let map = parse_input(&input);
        let position = (0,22);
        let expected_neighbours = HashSet::from([(0, 21), (1, 22)]);
        assert_eq!(expected_neighbours, neighbours(position, &map));
    }

    #[test]
    fn test_neighbours_bottom_right_corner() {
        let input = fs::read_to_string("23_example.txt").unwrap();
        let map = parse_input(&input);
        let position = (22,22);
        let expected_neighbours = HashSet::from([(21, 22), (22, 21)]);
        assert_eq!(expected_neighbours, neighbours(position, &map));
    }

    #[test]
    fn test_neighbours_bottom_row() {
        let input = fs::read_to_string("23_example.txt").unwrap();
        let map = parse_input(&input);
        let position = (10,22);
        let expected_neighbours = HashSet::from([(9, 22), (11, 22), (10, 21)]);
        assert_eq!(expected_neighbours, neighbours(position, &map));
    }

    #[test]
    fn test_neighbours_top_row() {
        let input = fs::read_to_string("23_example.txt").unwrap();
        let map = parse_input(&input);
        let position = (10,0);
        let expected_neighbours = HashSet::from([(9, 0), (11, 0), (10, 1)]);
        assert_eq!(expected_neighbours, neighbours(position, &map));
    }

    #[test]
    fn test_neighbours_left_column() {
        let input = fs::read_to_string("23_example.txt").unwrap();
        let map = parse_input(&input);
        let position = (0,10);
        let expected_neighbours = HashSet::from([(0, 9), (0, 11), (1, 10)]);
        assert_eq!(expected_neighbours, neighbours(position, &map));
    }

    #[test]
    fn test_neighbours_right_column() {
        let input = fs::read_to_string("23_example.txt").unwrap();
        let map = parse_input(&input);
        let position = (22,10);
        let expected_neighbours = HashSet::from([(22, 9), (22, 11), (21, 10)]);
        assert_eq!(expected_neighbours, neighbours(position, &map));
    }

    #[test]
    fn test_neighbours_regular_tile() {
        let input = fs::read_to_string("23_example.txt").unwrap();
        let map = parse_input(&input);
        let position = (10,10);
        let expected_neighbours = HashSet::from([(9, 10), (11, 10), (10, 9), (10, 11)]);
        assert_eq!(expected_neighbours, neighbours(position, &map));
    }
}
