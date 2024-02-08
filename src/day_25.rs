// use std::collections::hash_map::DefaultHasher;
use std::cmp::{max, min};
// use std::hash::{Hash, Hasher};
use std::fs;
use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = fs::read_to_string("25_example.txt").unwrap();
    println!("pt1 example: {}", pt1(&input));
}

fn pt1(input: &str) -> usize {
    let mut edges = HashSet::new();
    let mut vertices = HashSet::new();

    for line in input.lines() {
        let (component_name, connection_names) = line.split_once(": ").unwrap();
        // let (connection_names, weight) = connection_names.split_once(" ~ ").unwrap();
        // let weight = weight.parse().unwrap();

        vertices.insert(component_name.to_string());

        for connection_name in connection_names.split(" ") {
            let component_name = component_name.to_string();

            let connection_name = connection_name.to_string();

            vertices.insert(connection_name.clone());

            // TODO: move this logic to a tuple Struct constructor
            let weight = 1;
            let edge = (weight, min(component_name.clone(), connection_name.clone()), max(component_name, connection_name));
            edges.insert(edge);
        };
    }

    let mut vertices: Vec<String> = vertices.iter().map(|s| s.to_string()).collect();

    let mut min_weight = i32::MAX;
    let mut vertices_at_min_weight = vec![];

    let mut phase_count = 0;

    while vertices.len() > 1 {
        phase_count += 1;
        println!("Starting phase {phase_count} >>>\n\n");
        let weight = run_minimum_cut_phase(&mut vertices, &mut edges);

        if weight < min_weight {
            min_weight = weight;
            vertices_at_min_weight = vertices.clone();
        }
        println!();
    }

    println!("min_weight: {min_weight}");

    let group_1 = vertices_at_min_weight.pop().unwrap();
    let mut group_1: Vec<_> = group_1.split("-").collect();
    let mut group_2: Vec<_> = vertices_at_min_weight.iter().flat_map(|vertex| vertex.split("-")).collect();

    group_2.push(group_1.remove(0));

    // println!("group_1: {:?}", group_1);
    // println!("group_2: {:?}", group_2);

    group_1.len() * group_2.len()
}

fn run_minimum_cut_phase(vertexes: &mut Vec<String>, edges: &mut HashSet<(i32, String, String)>) -> i32 {
    let mut minimum_cut_phase = vec![];

    let starting_vertex = vertexes.remove(0);

    println!("starting_vertex: {starting_vertex}");

    minimum_cut_phase.push(starting_vertex);

    // println!("Start minimum_cut_phase: {:?}", minimum_cut_phase);
    // println!("Start vertexes: {:?}", vertexes);

    while !vertexes.is_empty() {
        vertexes.sort_by_key(|vertex| {
            let mut score = 0;

            for edge in edges.iter() {
                if edge.1 == *vertex && minimum_cut_phase.contains(&edge.2) {
                    score += edge.0;
                } else if edge.2 == *vertex && minimum_cut_phase.contains(&edge.1) {
                    score += edge.0;
                }
            }

            score
        });

        let next_vertex = vertexes.pop().unwrap();

        minimum_cut_phase.push(next_vertex);
    }

    // println!("Sorted minimum_cut_phase: {:?}", minimum_cut_phase);

    let t = minimum_cut_phase.pop().unwrap();

    // println!("t: {t}");

    let s = minimum_cut_phase.pop().unwrap();

    // println!("s: {s}");

    let s_t_edge = edges.iter().find(|edge| {
        (edge.1 == s && edge.2 == t) || (edge.2 == s && edge.1 == t)
    }).unwrap().clone();

    // println!("s_t_edge: {:?}", s_t_edge);

    edges.remove(&s_t_edge);

    let t_edges: HashSet<_> = edges.iter().filter(|edge| {
        edge.1 == t || edge.2 == t
    }).map(|edge| edge.clone()).collect();

    // println!("t_edges: {:?}", t_edges);

    let weight = t_edges.iter().map(|edge| edge.0).sum::<i32>() + s_t_edge.0;

    println!("weight: {weight}");

    for edge in &t_edges {
        edges.remove(edge);
    }

    // println!("edges after removing t edges: {:?}", edges);

    let s_edges: HashSet<_> = edges.iter().filter(|edge| {
        edge.1 == s || edge.2 == s
    }).map(|edge| edge.clone()).collect();

    // println!("s_edges: {:?}", s_edges);

    for edge in &s_edges {
        edges.remove(edge);
    }

    // println!("edges after removing s edges: {:?}", edges);

    let new_vertex = format!("{s}-{t}");

    let mut new_edges_weights = HashMap::new();

    for edge in &t_edges {
        let other_vertex = if edge.1 == t {
            &edge.2
        } else {
            &edge.1
        };

        let entry = new_edges_weights.entry(other_vertex).or_insert(0);

        let weight = edge.0;

        *entry += weight;
    }

    for edge in &s_edges {
        let other_vertex = if edge.1 == s {
            &edge.2
        } else {
            &edge.1
        };

        let entry = new_edges_weights.entry(other_vertex).or_insert(0);

        let weight = edge.0;

        *entry += weight;
    }

    // println!("new_edges_weights: {:?}", new_edges_weights);

    for (other_vertex, weight) in new_edges_weights {
        let new_edge = (weight, min(new_vertex.clone(), other_vertex.clone()), max(new_vertex.clone(), other_vertex.clone()));
        edges.insert(new_edge);
    }

    // println!("edges after adding new edges: {:?}", edges);

    for vertex in minimum_cut_phase {
        vertexes.push(vertex);
    }

    // println!("Adding new vertex: {new_vertex}");

    vertexes.push(new_vertex);

    // println!("vertexes after creating new vertex: {:?}", vertexes);

    weight
}

// fn hash(s: &str) -> u64 {
//     let mut hasher = DefaultHasher::new();
//     s.hash(&mut hasher);
//     hasher.finish()
// }

#[cfg(test)]
mod day_25_pt1_tests {
    use super::*;
}
