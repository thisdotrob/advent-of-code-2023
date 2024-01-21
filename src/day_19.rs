use std::fs;

pub fn run() {
    let example_input = fs::read_to_string("19_example.txt").unwrap();
    println!("pt1 example: {}", pt1(&example_input));
    // println!("pt2 example: {}", pt2(&example_input));
    // let input = fs::read_to_string("18.txt").unwrap();
    // println!("pt1: {}", pt1(&input));
    // println!("pt2: {}", pt2(&input));
}

fn pt1(input: &str) -> i32 {
    let (workflows, parts) = parse_input(input);
    println!("{:?}", workflows);
    println!("{:?}", parts);
    0
}

fn pt2(input: &str) -> i32 {
    0
}

fn parse_input(input: &str) -> (Vec<Workflow>, Vec<Part>) {
    let (workflows_str, parts_str) = input.split_once("\n\n").unwrap();
    let rules = parse_workflows(workflows_str);
    let parts = parse_parts(parts_str);
    (rules, parts)
}

fn parse_workflows(workflows_str: &str) -> Vec<Workflow> {
    workflows_str.lines().map(|line| {
        let (name, rest) = line.split_once("{").unwrap();
        let rules_str = &rest[..rest.len() - 1]; // remove the trailing }
        let rules = parse_rules(rules_str);  
        Workflow { name, rules }
    }).collect()
}

fn parse_parts(parts_str: &str) -> Vec<Part> {
    parts_str.lines().map(|line| {
        let mut part = Part { x: 0, m: 0, a: 0, s: 0 };
        let line = &line[1..line.len() - 1];
        for s in line.split(",") {
            let (property, value) = s.split_once("=").unwrap();
            match property {
                "x" => part.x = value.parse().unwrap(),
                "m" => part.m = value.parse().unwrap(),
                "a" => part.a = value.parse().unwrap(),
                "s" => part.s = value.parse().unwrap(),
                _ => panic!("invalid part property")
            }
        }
        part
    }).collect()
}

fn parse_rules(rules_str: &str) -> Vec<Rule> {
    rules_str.split(",").map(|rule_str| {
        if let Some((condition_str, destination_workflow)) = rule_str.split_once(":") {
            let condition = parse_condition(condition_str);
            Rule { destination_workflow, condition: Some(condition) }
        } else {
            Rule {destination_workflow: rule_str, condition: None }
        }
    }).collect()
}

fn parse_condition(condition_str: &str) -> RuleCondition {
    let mut chars = condition_str.chars();
    let part_property = chars.next().unwrap();
    let operator = chars.next().unwrap();
    let value = &condition_str[2..];
    let value = value.parse().unwrap();

    RuleCondition { part_property, operator, value }
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug)]
struct Rule<'a> {
    destination_workflow: &'a str,
    condition: Option<RuleCondition>,
}

#[derive(Debug)]
struct RuleCondition {
    part_property: char,
    operator: char,
    value: i32,
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}
