use std::fs;

pub fn run() {
    let example_input = fs::read_to_string("18_example.txt").unwrap();
    println!("pt1 example: {}", pt1(&example_input));
    let input = fs::read_to_string("18.txt").unwrap();
    println!("pt1: {}", pt1(&input));
}

fn pt1(input: &str) -> i32 {
    let mut trench_positions = vec![(0, 0)];
    let mut perimiter_count = 0;
    let mut pos = (0, 0);
    for line in input.lines() {
        let mut line = line.split(" ");
        let direction = line.next().unwrap();
        let meters: i32 = line.next().unwrap().parse().unwrap();
        let modifier = match direction {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!("invalid direction"),
        };

        pos.0 += modifier.0 * meters;
        pos.1 += modifier.1 * meters;
        trench_positions.push(pos);
        perimiter_count += meters;
    }
    let windows = trench_positions.windows(2).collect::<Vec<_>>();

    let mut area = 0;

    for window in windows {
        let (x1, y1) = window[0];
        let (x2, y2) = window[1];

        area += (x2 - x1) * (y2 + y1);
    }

    1 + ((area.abs() + perimiter_count) / 2)
}
