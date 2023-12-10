mod pt1;
mod pt2;

pub fn run() {
    println!("pt1: {:?}", pt1::run("7.txt"));
    println!("pt2: {:?}", pt2::run("7.txt"));
}

#[cfg(test)]
mod pt1_tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(6440, pt1::run("7_example.txt"));
    }
}

#[cfg(test)]
mod pt2_tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(5905, pt2::run("7_example.txt"));
    }
}
