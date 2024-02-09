use std::collections::hash_map::DefaultHasher;
use std::cmp::{max, min};
use std::hash::{Hash, Hasher};
use std::fs;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = fs::read_to_string("25_example.txt").unwrap();
    println!("pt1 example: {}", pt1(&input));
}

fn pt1(input: &str) -> usize {
    let mut components = create_components(input);

    let mut wires = create_wires(&components);

    let start_component = components.keys().next().unwrap().clone();

    loop {
        let cut_weight = phase(&mut components, &mut wires, start_component);
        if cut_weight == 3 {
            break
        }
    }

    0
}

fn phase(components: &mut HashMap<u64, Vec<u64>>, wires: &mut HashMap<(u64, u64), u64>, start_component: u64) -> u64 {
    let ordered_components = ordered_components(components, wires, start_component);

    let t = ordered_components[ordered_components.len() - 1];
    let s = ordered_components[ordered_components.len() - 2];

    let t_connections = components.remove(&t).unwrap();
    let s_connections = components.remove(&s).unwrap();

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

    components.insert(new_component_hash, new_connection_hashes);

    cut_weight
}

fn ordered_components(components: &HashMap<u64, Vec<u64>>, wires: &HashMap<(u64, u64), u64>, start_component: u64) -> Vec<u64> {
    let mut ordered_components = vec![start_component];

    while ordered_components.len() != components.len() {
        let mut connected_components: HashMap<u64, u64> = HashMap::new();

        for component_hash_a in &ordered_components {
            let connections = components.get(component_hash_a).unwrap();
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

fn create_components(input: &str) -> HashMap<u64, Vec<u64>> {
    let mut components: HashMap<u64, Vec<u64>> = HashMap::new();

    for line in input.lines() {
        let (component_name, rest) = line.split_once(": ").unwrap();
        let component_hash = hash(component_name);
        let mut connection_hashes: Vec<_> = rest.split(" ").map(|s| hash(s)).collect();
        for connection_hash in &connection_hashes {
            let connected_component = components.entry(*connection_hash).or_insert(vec![]);
            connected_component.push(component_hash);
        }
        let component = components.entry(component_hash).or_insert(vec![]);
        component.append(&mut connection_hashes);
    }

    for connected_component in components.values() {
        let as_set: HashSet<_> = connected_component.iter().collect();
        assert!(connected_component.len() == as_set.len());
    }

    components
}

fn create_wires(components: &HashMap<u64, Vec<u64>>) -> HashMap<(u64, u64), u64> {
    let mut wires = HashMap::new();

    for (component_hash_a, connected_component_hashes) in components {
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
