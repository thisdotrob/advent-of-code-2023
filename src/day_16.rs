use std::fs;

pub fn run() {
    let example_input = fs::read_to_string("16_example.txt").unwrap();
    println!("pt1 example: {}", pt1::<10>(&example_input));
    let input = fs::read_to_string("16.txt").unwrap();
    println!("pt1: {}", pt1::<110>(&input));
}

fn pt1<const N: usize>(input: &str) -> usize {
    let sum = 0;

    let contraption: Contraption<N> = Contraption::from(input);

    dbg!(contraption.tiles);

    sum
}

struct Contraption<const N: usize> {
    tiles: [[char; N]; N],
}

impl<const N: usize> From<&str> for Contraption<N> {
    fn from(s: &str) -> Self {
        let mut lines = s.lines().map(|l| l.chars());

        Self {
            tiles: std::array::from_fn(|_| {
                let mut line = lines.next().unwrap();
                std::array::from_fn(|_| line.next().unwrap())
            }),
        }
    }
}
