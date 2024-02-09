use std::collections::hash_map::DefaultHasher;
use std::cmp::{max, min};
use std::hash::{Hash, Hasher};
use std::fs;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = fs::read_to_string("25_example.txt").unwrap();
    println!("pt1 example: {}", pt1(&input));
}

fn pt1(input: &str) -> u64 {
    let mut components = create_components(input);

    let mut wires = create_wires(&components);

    // let start_component = components.keys().next().unwrap().clone();
    let start_component = 473070775585238438;

    // let total_component_count: u64 = components.len().try_into().unwrap();

    let cut_component_count = find_min_cut(&mut components, &mut wires, start_component);

    cut_component_count
}

fn find_min_cut(components: &mut HashMap<u64, (Vec<u64>, u64)>, wires: &mut HashMap<(u64, u64), u64>, start_component: u64) -> u64 {
    loop {
        let (cut_weight, cut_component_count) = phase(components, wires, start_component);
        if cut_weight == 3 {
            return cut_component_count
        }
    }
}

fn phase(components: &mut HashMap<u64, (Vec<u64>, u64)>, wires: &mut HashMap<(u64, u64), u64>, start_component: u64) -> (u64, u64) {
    let ordered_components = ordered_components(components, wires, start_component);

    let t = ordered_components[ordered_components.len() - 1];
    let s = ordered_components[ordered_components.len() - 2];

    let (t_connections, t_component_count) = components.remove(&t).unwrap();
    let (s_connections, s_component_count) = components.remove(&s).unwrap();

    let mut new_connections = HashMap::new();

    let mut cut_weight = 0;

    for connection_hash in t_connections {
        if connection_hash == s {
            continue
        }
        let new_connection = new_connections.entry(connection_hash).or_insert(0);
        let wire = wire(t, connection_hash);
        let wire_weight = wires.get(&wire).unwrap();
        *new_connection += wire_weight;
        cut_weight += wire_weight;
        wires.remove(&wire);
    }

    for connection_hash in s_connections {
        if connection_hash == t {
            continue
        }
        let new_connection = new_connections.entry(connection_hash).or_insert(0);
        let wire = wire(s, connection_hash);
        let wire_weight = wires.get(&wire).unwrap();
        *new_connection += wire_weight;
        wires.remove(&wire);
    }

    let new_component_hash = hash(&format!("{t}{s}")); // TODO: think of a better way to generate a
                                                       // new hash
    for (connection_hash, wire_weight) in &new_connections {
        let wire = wire(new_component_hash, *connection_hash);
        wires.insert(wire, *wire_weight);
    }

    let new_connection_hashes = new_connections.into_keys().collect();

    let new_component_count = t_component_count + s_component_count;

    components.insert(new_component_hash, (new_connection_hashes, new_component_count));

    (cut_weight, new_component_count)
}

fn ordered_components(components: &HashMap<u64, (Vec<u64>, u64)>, wires: &HashMap<(u64, u64), u64>, start_component: u64) -> Vec<u64> {
    let mut ordered_components = vec![start_component];

    while ordered_components.len() != components.len() {
        let mut connected_components: HashMap<u64, u64> = HashMap::new();

        for component_hash_a in &ordered_components {
            let (connections, _) = components.get(component_hash_a).unwrap();
            for component_hash_b in connections {
                let wire = wire(*component_hash_a, *component_hash_b);
                let wire_weight = wires.get(&wire).unwrap();
                let entry = connected_components.entry(*component_hash_b).or_insert(0);
                *entry += wire_weight;
            }
        }

        let next_component = connected_components.iter().max_by_key(|(_, weight)| *weight).unwrap();

        let next_component_hash = next_component.0;

        ordered_components.push(*next_component_hash);
    }

    ordered_components
}

fn create_components(input: &str) -> HashMap<u64, (Vec<u64>, u64)> {
    let mut components: HashMap<u64, (Vec<u64>, u64)> = HashMap::new();
    let mut hash_lookup: HashMap<u64, &str> = HashMap::new();

    for line in input.lines() {
        let (component_name, rest) = line.split_once(": ").unwrap();
        let component_hash = hash(component_name);
        hash_lookup.insert(component_hash, component_name);
        let mut connection_hashes: Vec<_> = rest.split(" ").map(|s| {
            let h = hash(s);
            hash_lookup.insert(h, s);
            h
        }).collect();
        for connection_hash in &connection_hashes {
            let connected_component = components.entry(*connection_hash).or_insert((vec![], 1));
            connected_component.0.push(component_hash);
        }
        let component = components.entry(component_hash).or_insert((vec![], 1));
        component.0.append(&mut connection_hashes);
    }

    for connected_component in components.values() {
        let as_set: HashSet<_> = connected_component.0.iter().collect();
        assert!(connected_component.0.len() == as_set.len());
    }

    println!("{:?}", hash_lookup);

    components
}

fn create_wires(components: &HashMap<u64, (Vec<u64>, u64)>) -> HashMap<(u64, u64), u64> {
    let mut wires = HashMap::new();

    for (component_hash_a, (connected_component_hashes, _)) in components {
        let new_wires = connected_component_hashes.iter().map(|component_hash_b| wire(*component_hash_a, *component_hash_b));
        for wire in new_wires {
            let weight = 1;
            wires.insert(wire, weight);
        }
    }

    wires
}

fn hash(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

fn wire(component_hash_a: u64, component_hash_b: u64) -> (u64, u64) {
    (min(component_hash_a, component_hash_b), max(component_hash_a, component_hash_b))
}

#[cfg(test)]
mod day_25_tests {
    use super::*;

    #[test]
    fn test_paper_example() {
        let mut components = HashMap::new();
        components.insert(1, (vec![2, 5], 1));
        components.insert(2, (vec![1, 3, 5, 6], 1));
        components.insert(3, (vec![2, 4, 7], 1));
        components.insert(4, (vec![3, 7, 8], 1));
        components.insert(5, (vec![1, 2, 6], 1));
        components.insert(6, (vec![2, 5, 7], 1));
        components.insert(7, (vec![3, 4, 6, 8], 1));
        components.insert(8, (vec![4, 7], 1));

        let mut wires = HashMap::new();
        wires.insert((1, 2), 2);
        wires.insert((2, 3), 3);
        wires.insert((3, 4), 4);
        wires.insert((4, 8), 2);
        wires.insert((7, 8), 3);
        wires.insert((6, 7), 1);
        wires.insert((5, 6), 3);
        wires.insert((1, 5), 3);
        wires.insert((2, 6), 2);
        wires.insert((3, 7), 2);
        wires.insert((2, 5), 2);
        wires.insert((4, 7), 2);

        let start_component = 2;

        let cut_component_count = find_min_cut(&mut components, &mut wires, start_component);

        assert_eq!(4, cut_component_count);
    }
}
