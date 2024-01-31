use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("23_example.txt").unwrap();
    println!("pt1 example: {}", pt1(&input));
    println!("pt2 example: {}", pt2(&input));
    let input = fs::read_to_string("23.txt").unwrap();
    println!("pt1: {}", pt1(&input));
    println!("pt2: {}", pt2(&input));
}

fn pt1(input: &str) -> usize {
    let slopes_are_climbable = false;
    longest_hike(input, slopes_are_climbable)
}

fn pt2(input: &str) -> usize {
    let slopes_are_climbable = true;
    longest_hike(input, slopes_are_climbable)
}

fn longest_hike(input: &str, slopes_are_climbable: bool) -> usize {
    let map = parse_input(input);

    let start_position = find_start_position(&map);

    let end_position = find_end_position(&map);

    let graph = construct_graph(&map, start_position, end_position, slopes_are_climbable);

    let mut heap = BinaryHeap::new();

    let start_distance = 0;
    let visited = vec![];
    heap.push((start_distance, &start_position, visited));

    let mut longest_start_to_end_distance = 0;

    while let Some((distance, position, visited)) = heap.pop() {
        if *position == end_position {
            if distance > longest_start_to_end_distance {
                longest_start_to_end_distance = distance;
            }
        }

        for (next_node, next_node_distance) in graph.get(&position).unwrap() {
            if !visited.contains(&next_node) {
                let mut visited = visited.clone();
                visited.push(&position);
                heap.push((distance + next_node_distance, next_node, visited));
            }
        }
    }

    longest_start_to_end_distance
}

fn construct_graph(
    map: &Vec<Vec<char>>,
    start_position: (usize, usize),
    end_position: (usize, usize),
    slopes_are_climbable: bool,
) -> HashMap<(usize, usize), HashMap<(usize, usize), usize>> {
    let mut graph = HashMap::new();

    let mut graph_nodes = find_junctions(&map, slopes_are_climbable);

    graph_nodes.push(start_position);
    graph_nodes.push(end_position);

    for graph_node in &graph_nodes {
        graph.insert(
            *graph_node,
            calculate_junction_distances(&graph_node, &graph_nodes, &map, slopes_are_climbable),
        );
    }

    graph
}

// TODO: rename to calculate_graph_node_distances
fn calculate_junction_distances(
    junction: &(usize, usize),
    junctions: &Vec<(usize, usize)>,
    map: &Vec<Vec<char>>,
    slopes_are_climbable: bool,
) -> HashMap<(usize, usize), usize> {
    let mut distances = HashMap::new();

    // TODO: pass in de-referenced junction
    let mut seen = HashSet::from([*junction]);

    let mut positions_to_visit: Vec<_> = neighbours(*junction, map, slopes_are_climbable)
        .iter()
        .map(|position| {
            let distance = 1;
            (*position, distance)
        })
        .collect();

    while let Some((position, distance)) = positions_to_visit.pop() {
        if seen.contains(&position) {
            continue;
        }

        seen.insert(position);

        // TODO: make junctions a HashSet
        if junctions.contains(&position) {
            distances.insert(position, distance);
        } else {
            for position in neighbours(position, map, slopes_are_climbable) {
                positions_to_visit.push((position, distance + 1));
            }
        }
    }

    distances
}

// TODO: rewrite this to use a map and filter
fn find_junctions(map: &Vec<Vec<char>>, slopes_are_climbable: bool) -> Vec<(usize, usize)> {
    let mut junctions = vec![];
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == '#' {
                continue;
            } else if neighbour_count((x, y), map, slopes_are_climbable) > 2 {
                junctions.push((x, y));
            }
        }
    }
    junctions
}

fn neighbours(
    position: (usize, usize),
    map: &Vec<Vec<char>>,
    slopes_are_climbable: bool,
) -> Vec<(usize, usize)> {
    let (x, y) = position;

    let mut neighbours = vec![];

    let mut tile = map[y][x];

    if slopes_are_climbable {
        tile = '.';
    }

    match tile {
        '<' => {
            if x > 0 && map[y][x - 1] != '#' {
                neighbours.push((x - 1, y));
            }
        }
        '^' => {
            if y > 0 && map[y - 1][x] != '#' {
                neighbours.push((x, y - 1));
            }
        }
        '>' => {
            if x < (map[0].len() - 1) && map[y][x + 1] != '#' {
                neighbours.push((x + 1, y));
            }
        }
        'v' => {
            if y < (map.len() - 1) && map[y + 1][x] != '#' {
                neighbours.push((x, y + 1));
            }
        }
        _ => {
            if x > 0 && map[y][x - 1] != '#' {
                neighbours.push((x - 1, y));
            }

            if y > 0 && map[y - 1][x] != '#' {
                neighbours.push((x, y - 1));
            }

            if x < (map[0].len() - 1) && map[y][x + 1] != '#' {
                neighbours.push((x + 1, y));
            }

            if y < (map.len() - 1) && map[y + 1][x] != '#' {
                neighbours.push((x, y + 1));
            }
        }
    }

    neighbours
}

fn neighbour_count(
    position: (usize, usize),
    map: &Vec<Vec<char>>,
    slopes_are_climbable: bool,
) -> usize {
    neighbours(position, map, slopes_are_climbable).len()
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

#[cfg(test)]
mod day_23_pt_2_tests {
    use super::*;

    #[test]
    fn test_find_junctions() {
        let input = fs::read_to_string("23_example.txt").unwrap();
        let map = parse_input(&input);
        let junctions = find_junctions(&map, true);
        let expected_junctions = vec![
            (11, 3),
            (3, 5),
            (21, 11),
            (5, 13),
            (13, 13),
            (13, 19),
            (19, 19),
        ];
        assert_eq!(expected_junctions, junctions);
    }

    #[test]
    fn test_calculate_junction_distances() {
        let input = fs::read_to_string("23_example.txt").unwrap();
        let map = parse_input(&input);
        let junctions = find_junctions(&map, true);
        let junction = (3, 5);
        let junction_distances = calculate_junction_distances(&junction, &junctions, &map, true);
        let mut expected_junction_distances = HashMap::new();
        expected_junction_distances.insert((5, 13), 22);
        expected_junction_distances.insert((11, 3), 22);
        assert_eq!(expected_junction_distances, junction_distances);
    }

    #[test]
    fn test_binary_heap() {
        let mut heap = BinaryHeap::new();
        heap.push((0, (1, 0)));
        heap.push((0, (1, 0)));
    }

    #[test]
    fn test_construct_graph() {
        let input = fs::read_to_string("23_example.txt").unwrap();
        let map = parse_input(&input);
        let start_position = find_start_position(&map);
        let end_position = find_end_position(&map);
        let graph = construct_graph(&map, start_position, end_position, true);

        let node = graph.get(&(1, 0)).unwrap();
        assert_eq!(1, node.len());
        assert_eq!(15, *node.get(&(3, 5)).unwrap());

        let node = graph.get(&(3, 5)).unwrap();
        assert_eq!(3, node.len());
        assert_eq!(15, *node.get(&(1, 0)).unwrap());
        assert_eq!(22, *node.get(&(11, 3)).unwrap());
        assert_eq!(22, *node.get(&(5, 13)).unwrap());

        let node = graph.get(&(11, 3)).unwrap();
        assert_eq!(3, node.len());
        assert_eq!(22, *node.get(&(3, 5)).unwrap());
        assert_eq!(24, *node.get(&(13, 13)).unwrap());
        assert_eq!(30, *node.get(&(21, 11)).unwrap());

        let node = graph.get(&(5, 13)).unwrap();
        assert_eq!(3, node.len());
        assert_eq!(22, *node.get(&(3, 5)).unwrap());
        assert_eq!(12, *node.get(&(13, 13)).unwrap());
        assert_eq!(38, *node.get(&(13, 19)).unwrap());

        let node = graph.get(&(13, 13)).unwrap();
        assert_eq!(4, node.len());
        assert_eq!(24, *node.get(&(11, 3)).unwrap());
        assert_eq!(12, *node.get(&(5, 13)).unwrap());
        assert_eq!(10, *node.get(&(13, 19)).unwrap());
        assert_eq!(18, *node.get(&(21, 11)).unwrap());

        let node = graph.get(&(13, 19)).unwrap();
        assert_eq!(3, node.len());
        assert_eq!(10, *node.get(&(13, 13)).unwrap());
        assert_eq!(38, *node.get(&(5, 13)).unwrap());
        assert_eq!(10, *node.get(&(19, 19)).unwrap());

        let node = graph.get(&(21, 11)).unwrap();
        assert_eq!(3, node.len());
        assert_eq!(18, *node.get(&(13, 13)).unwrap());
        assert_eq!(30, *node.get(&(11, 3)).unwrap());
        assert_eq!(10, *node.get(&(19, 19)).unwrap());

        let node = graph.get(&(19, 19)).unwrap();
        assert_eq!(3, node.len());
        assert_eq!(10, *node.get(&(13, 19)).unwrap());
        assert_eq!(10, *node.get(&(21, 11)).unwrap());
        assert_eq!(5, *node.get(&(21, 22)).unwrap());
    }
}
