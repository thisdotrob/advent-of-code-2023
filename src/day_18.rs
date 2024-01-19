use std::fs;

pub fn run() {
    let example_input = fs::read_to_string("18_example.txt").unwrap();
    println!("pt1 example: {}", pt1(&example_input));
}

fn pt1(input: &str) -> usize {
    let mut trench_positions = vec![];
    let mut pos = (0, 0);
    for line in input.lines() {
        let mut line = line.split(" ");
        let direction = line.next().unwrap();
        let meters: usize = line.next().unwrap().parse().unwrap();
        let modifier = match direction {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!("invalid direction")
        };
        for _ in 0..meters {
            pos.0 += modifier.0;
            pos.1 += modifier.1;

            trench_positions.push(pos);
        }
    }

    println!("{:?}", trench_positions);

    let values_of_x = trench_positions.iter().map(|position| position.0);

    println!("{:?}", values_of_x.clone().collect::<Vec<_>>());

    let max_x = values_of_x.max().unwrap();

    println!("{:?}", max_x);

    let max_y = trench_positions.iter().map(|position| position.1).max().unwrap();

    for x in 0..max_x {
        dbg!(x);
        let mut trench_positions_for_x: Vec<_> = trench_positions.iter().filter_map(|pos| {
            if pos.0 == x {
                Some(pos.1)
            } else {
                None
            }
        }).collect();

        trench_positions_for_x.sort_unstable();

        let mut prev_pos = trench_positions_for_x.pop().unwrap();
        while let Some(pos) = trench_positions_for_x.pop() {
            dbg!(prev_pos);
            dbg!(pos);

            prev_pos = pos;
        }
    }

    0
}
