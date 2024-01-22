use std::collections::HashMap;
use std::fs;

pub fn run() {
    let example_input = fs::read_to_string("19_example.txt").unwrap();
    println!("pt1 example: {}", pt1(&example_input));
    println!("pt2 example: {}", pt2(&example_input));
    let input = fs::read_to_string("19.txt").unwrap();
    println!("pt1: {}", pt1(&input));
    println!("pt2: {}", pt2(&input));
}

fn pt1(input: &str) -> u64 {
    let (workflows, parts) = parse_input(input);

    let mut answer = 0;

    for part in parts {
        let mut workflow_name = "in";

        loop {
            if workflow_name == "R" {
                break;
            } else if workflow_name == "A" {
                answer += part.x + part.m + part.a + part.s;
                break;
            }

            let workflow = &workflows[workflow_name];

            for rule in &workflow.rules {
                if let Some(condition) = &rule.condition {
                    let part_value = match condition.part_property {
                        'x' => part.x,
                        'm' => part.m,
                        'a' => part.a,
                        's' => part.s,
                        _ => panic!("invalid part property"),
                    };

                    let result = match condition.operator {
                        '>' => part_value > condition.value,
                        '<' => part_value < condition.value,
                        _ => panic!("invalid condition operator"),
                    };

                    if result {
                        workflow_name = rule.destination_workflow;
                        break;
                    } else {
                        continue;
                    }
                } else {
                    workflow_name = rule.destination_workflow;
                    break;
                }
            }
        }
    }

    answer
}

fn pt2(input: &str) -> u64 {
    let (workflows, _) = parse_input(input);

    let part_range = PartRange {
        x: [1, 4000],
        m: [1, 4000],
        a: [1, 4000],
        s: [1, 4000],
    };

    let accepted_part_ranges = apply_workflow(&part_range, &workflows, "in", 0);

    let mut answer = 0;

    for range in accepted_part_ranges {
        let num_distinct_combinations = (1 + range.x[1] - range.x[0])
            * (1 + range.m[1] - range.m[0])
            * (1 + range.a[1] - range.a[0])
            * (1 + range.s[1] - range.s[0]);
        answer += num_distinct_combinations;
    }

    answer
}

fn apply_workflow(
    part_range: &PartRange,
    workflows: &HashMap<&str, Workflow>,
    destination_workflow: &str,
    rule_index: usize,
) -> Vec<PartRange> {
    let workflow = &workflows[destination_workflow];

    if rule_index == workflow.rules.len() {
        return vec![];
    }

    let rule = &workflow.rules[rule_index];

    let mut result = vec![];

    if let Some(matching_part_range) = &filter_part_range(part_range, rule) {
        if rule.destination_workflow == "A" {
            result.push(PartRange {
                ..*matching_part_range
            });
        } else if rule.destination_workflow != "R" {
            result.append(&mut apply_workflow(
                matching_part_range,
                workflows,
                rule.destination_workflow,
                0,
            ));
        }
        if let Some(non_matching_part_range) =
            &part_range_remainder(part_range, matching_part_range)
        {
            result.append(&mut apply_workflow(
                non_matching_part_range,
                workflows,
                destination_workflow,
                rule_index + 1,
            ))
        }
    } else {
        result.append(&mut apply_workflow(
            part_range,
            workflows,
            destination_workflow,
            rule_index + 1,
        ))
    }

    result
}

fn filter_part_range(part_range: &PartRange, rule: &Rule) -> Option<PartRange> {
    if let Some(condition) = &rule.condition {
        let filtered_part_range = if condition.operator == '>' {
            match condition.part_property {
                'x' => PartRange {
                    x: [condition.value + 1, part_range.x[1]],
                    ..*part_range
                },
                'm' => PartRange {
                    m: [condition.value + 1, part_range.m[1]],
                    ..*part_range
                },
                'a' => PartRange {
                    a: [condition.value + 1, part_range.a[1]],
                    ..*part_range
                },
                's' => PartRange {
                    s: [condition.value + 1, part_range.s[1]],
                    ..*part_range
                },
                _ => panic!("invalid part property"),
            }
        } else {
            // operator must be '<'
            match condition.part_property {
                'x' => PartRange {
                    x: [part_range.x[0], condition.value - 1],
                    ..*part_range
                },
                'm' => PartRange {
                    m: [part_range.m[0], condition.value - 1],
                    ..*part_range
                },
                'a' => PartRange {
                    a: [part_range.a[0], condition.value - 1],
                    ..*part_range
                },
                's' => PartRange {
                    s: [part_range.s[0], condition.value - 1],
                    ..*part_range
                },
                _ => panic!("invalid part property"),
            }
        };

        if part_range_is_valid(&filtered_part_range) {
            Some(filtered_part_range)
        } else {
            None
        }
    } else {
        Some(PartRange { ..*part_range })
    }
}

fn part_range_remainder(
    original_part_range: &PartRange,
    part_range: &PartRange,
) -> Option<PartRange> {
    let remainder = PartRange {
        x: range_remainder(original_part_range.x, part_range.x),
        m: range_remainder(original_part_range.m, part_range.m),
        a: range_remainder(original_part_range.a, part_range.a),
        s: range_remainder(original_part_range.s, part_range.s),
    };

    if part_range_is_valid(&remainder) {
        Some(remainder)
    } else {
        None
    }
}

fn range_remainder(original_range: [u64; 2], range: [u64; 2]) -> [u64; 2] {
    if range[1] != original_range[1] && range[0] != original_range[0] {
        panic!("both start and end of range are different");
    }

    if range[1] != original_range[1] {
        [range[1] + 1, original_range[1]]
    } else if range[0] != original_range[0] {
        [original_range[0], range[0] - 1]
    } else {
        original_range
    }
}

fn part_range_is_valid(part_range: &PartRange) -> bool {
    part_range.x[0] < part_range.x[1]
        && part_range.m[0] < part_range.m[1]
        && part_range.a[0] < part_range.a[1]
        && part_range.s[0] < part_range.s[1]
}

fn parse_input(input: &str) -> (HashMap<&str, Workflow>, Vec<Part>) {
    let (workflows_str, parts_str) = input.split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows_str);
    let parts = parse_parts(parts_str);
    (workflows, parts)
}

fn parse_workflows(workflows_str: &str) -> HashMap<&str, Workflow> {
    workflows_str
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once("{").unwrap();
            let rules_str = &rest[..rest.len() - 1]; // remove the trailing }
            let rules = parse_rules(rules_str);
            (name, Workflow { rules })
        })
        .collect()
}

fn parse_parts(parts_str: &str) -> Vec<Part> {
    parts_str
        .lines()
        .map(|line| {
            let mut part = Part {
                x: 0,
                m: 0,
                a: 0,
                s: 0,
            };
            let line = &line[1..line.len() - 1];
            for s in line.split(",") {
                let (property, value) = s.split_once("=").unwrap();
                match property {
                    "x" => part.x = value.parse().unwrap(),
                    "m" => part.m = value.parse().unwrap(),
                    "a" => part.a = value.parse().unwrap(),
                    "s" => part.s = value.parse().unwrap(),
                    _ => panic!("invalid part property"),
                }
            }
            part
        })
        .collect()
}

fn parse_rules(rules_str: &str) -> Vec<Rule> {
    rules_str
        .split(",")
        .map(|rule_str| {
            if let Some((condition_str, destination_workflow)) = rule_str.split_once(":") {
                let condition = parse_condition(condition_str);
                Rule {
                    destination_workflow,
                    condition: Some(condition),
                }
            } else {
                Rule {
                    destination_workflow: rule_str,
                    condition: None,
                }
            }
        })
        .collect()
}

fn parse_condition(condition_str: &str) -> RuleCondition {
    let mut chars = condition_str.chars();
    let part_property = chars.next().unwrap();
    let operator = chars.next().unwrap();
    let value = &condition_str[2..];
    let value = value.parse().unwrap();

    RuleCondition {
        part_property,
        operator,
        value,
    }
}

#[derive(Debug)]
struct Workflow<'a> {
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
    value: u64,
}

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug)]
struct PartRange {
    x: [u64; 2],
    m: [u64; 2],
    a: [u64; 2],
    s: [u64; 2],
}

#[cfg(test)]
mod day_19_pt2_tests {
    use super::*;

    #[test]
    fn test_minimal_input() {
        let input = "in{a<2001:A,R}\n\n";

        let expected = 2000 * 4000 * 4000 * 4000;
        assert_eq!(expected, pt2(input));
    }

    #[test]
    fn test_minimal_input_2() {
        let input = "in{a<2001:A,m>1000:A,R}\n\n";

        let expected = (2000 * 4000 * 4000 * 4000) + (2000 * 3000 * 4000 * 4000);
        assert_eq!(expected, pt2(input));
    }

    #[test]
    fn test_minimal_input_3() {
        let input = "in{a<2001:aaa,R}
aaa{A}\n\n";

        let expected = 2000 * 4000 * 4000 * 4000;
        assert_eq!(expected, pt2(input));
    }

    #[test]
    fn test_minimal_input_4() {
        let input = "in{s<1351:px,qqz}
px{a<2006:A,R}
qqz{A}\n\n";

        let mut expected = 4000 * 4000 * (4000 - 1350) * 4000;
        expected += 4000 * 4000 * 2005 * 1350; // px
        assert_eq!(expected, pt2(input));
    }
}
