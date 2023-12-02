use std::fs;

#[derive(PartialEq, Debug)]
struct GameSet {
    blue: u32,
    red: u32,
    green: u32,
}

#[derive(PartialEq, Debug)]
struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn run() {
    let contents = fs::read_to_string("2.txt").unwrap();

    let games: Vec<_> = contents.lines().map(|l| parse_game_record(l)).collect();

    let mut sum_of_possible_game_ids = 0;

    let mut sum_of_minimum_cube_powers = 0;

    for game in games {
        let mut possible = true;

        let mut min_red_needed = 0;
        let mut min_green_needed = 0;
        let mut min_blue_needed = 0;

        for set in game.sets {
            if set.red > MAX_RED || set.green > MAX_GREEN || set.blue > MAX_BLUE {
                possible = false;
            }

            if set.red > min_red_needed {
                min_red_needed = set.red;
            }

            if set.green > min_green_needed {
                min_green_needed = set.green;
            }

            if set.blue > min_blue_needed {
                min_blue_needed = set.blue;
            }
        }

        let minimum_cube_power = min_red_needed * min_green_needed * min_blue_needed;

        sum_of_minimum_cube_powers += minimum_cube_power;

        if possible {
            sum_of_possible_game_ids += game.id;
        }
    }

    println!("pt1: {}", sum_of_possible_game_ids);

    println!("pt2: {}", sum_of_minimum_cube_powers);
}

fn parse_game_set(s: &str) -> GameSet {
    let mut blue = "0";
    let mut red = "0";
    let mut green = "0";
    let cube_groups = s.split(",").map(|s| s.trim()).collect::<Vec<&str>>();
    for cube_group in cube_groups {
        let (count, colour) = cube_group.split_once(" ").unwrap();
        match colour {
            "blue" => blue = count,
            "red" => red = count,
            "green" => green = count,
            _ => panic!("Unrecognised colour: {}", colour),
        }
    }
    GameSet {
        blue: blue.parse::<u32>().unwrap(),
        red: red.parse::<u32>().unwrap(),
        green: green.parse::<u32>().unwrap(),
    }
}

fn parse_game_record(s: &str) -> Game {
    let (game_id, rest) = s.split_once(":").unwrap();
    let (_, game_id) = game_id.split_once(" ").unwrap();
    let game_sets = rest.split(";").map(|s| parse_game_set(s)).collect();
    Game {
        id: game_id.parse::<u32>().unwrap(),
        sets: game_sets,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_game_set_1() {
        let s = " 3 blue, 4 red";
        let game_set = parse_game_set(s);
        assert_eq!(
            game_set,
            GameSet {
                blue: 3,
                red: 4,
                green: 0
            }
        );
    }

    #[test]
    fn parse_game_set_2() {
        let s = " 1 red, 2 green, 6 blue";
        let game_set = parse_game_set(s);
        assert_eq!(
            game_set,
            GameSet {
                blue: 6,
                red: 1,
                green: 2
            }
        );
    }

    #[test]
    fn parse_game_set_3() {
        let s = " 2 green";
        let game_set = parse_game_set(s);
        assert_eq!(
            game_set,
            GameSet {
                blue: 0,
                red: 0,
                green: 2
            }
        );
    }

    #[test]
    fn parse_game_set_4() {
        let s = " 8 green, 6 blue, 20 red";
        let game_set = parse_game_set(s);
        assert_eq!(
            game_set,
            GameSet {
                blue: 6,
                red: 20,
                green: 8
            }
        );
    }

    #[test]
    fn parse_game_record_1() {
        let game_record_line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = parse_game_record(game_record_line);
        assert_eq!(
            game,
            Game {
                id: 1,
                sets: vec![
                    GameSet {
                        blue: 3,
                        red: 4,
                        green: 0
                    },
                    GameSet {
                        blue: 6,
                        red: 1,
                        green: 2
                    },
                    GameSet {
                        blue: 0,
                        red: 0,
                        green: 2
                    },
                ],
            }
        );
    }

    #[test]
    fn parse_game_record_2() {
        let game_record_line = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let game = parse_game_record(game_record_line);
        assert_eq!(
            game,
            Game {
                id: 2,
                sets: vec![
                    GameSet {
                        blue: 1,
                        red: 0,
                        green: 2
                    },
                    GameSet {
                        blue: 4,
                        red: 1,
                        green: 3
                    },
                    GameSet {
                        blue: 1,
                        red: 0,
                        green: 1
                    },
                ],
            }
        );
        assert_eq!(2, game.id);
    }

    #[test]
    fn parse_game_record_3() {
        let game_record_line =
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game = parse_game_record(game_record_line);
        assert_eq!(
            game,
            Game {
                id: 3,
                sets: vec![
                    GameSet {
                        blue: 6,
                        red: 20,
                        green: 8
                    },
                    GameSet {
                        blue: 5,
                        red: 4,
                        green: 13
                    },
                    GameSet {
                        blue: 0,
                        red: 1,
                        green: 5
                    },
                ],
            }
        );
    }

    #[test]
    fn parse_game_record_4() {
        let game_record_line =
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let game = parse_game_record(game_record_line);
        assert_eq!(
            game,
            Game {
                id: 4,
                sets: vec![
                    GameSet {
                        blue: 6,
                        red: 3,
                        green: 1
                    },
                    GameSet {
                        blue: 0,
                        red: 6,
                        green: 3
                    },
                    GameSet {
                        blue: 15,
                        red: 14,
                        green: 3
                    },
                ],
            }
        );
    }

    #[test]
    fn parse_game_record_5() {
        let game_record_line = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let game = parse_game_record(game_record_line);
        assert_eq!(
            game,
            Game {
                id: 5,
                sets: vec![
                    GameSet {
                        blue: 1,
                        red: 6,
                        green: 3
                    },
                    GameSet {
                        blue: 2,
                        red: 1,
                        green: 2
                    },
                ],
            }
        );
    }
}
