#![allow(dead_code, unused_imports)]
use std::fs;

const RED_LIMIT: usize = 12;
const GREEN_LIMIT: usize = 13;
const BLUE_LIMIT: usize = 14;

#[derive(Clone)]
struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Clone)]
struct Game {
    id: usize,
    game_results: Vec<Hand>,
}

fn get_game_data(game_str: &str) -> Game {
    let game_split: Vec<&str> = game_str.split(':').collect::<Vec<&str>>();

    let game_num_split: Vec<&str> = game_split[0].split(' ').collect();
    let game_id = game_num_split[1].parse::<usize>().unwrap();

    let mut hand_data: Vec<Hand> = Vec::new();

    for play in game_split[1].split(';') {
        let mut hand = Hand {
            red: 0,
            green: 0,
            blue: 0,
        };

        for set in play.split(',') {
            let set_split = set.trim().split(' ').collect::<Vec<&str>>();

            if set_split[1].trim() == "red" {
                hand.red = set_split[0].parse::<usize>().unwrap();
            }
            if set_split[1].trim() == "green" {
                hand.green = set_split[0].parse::<usize>().unwrap();
            }
            if set_split[1].trim() == "blue" {
                hand.blue = set_split[0].parse::<usize>().unwrap();
            }
        }

        hand_data.push(hand);
    }

    Game {
        id: game_id,
        game_results: hand_data,
    }
}

fn is_game_possible(game: &Game) -> bool {
    for play in &game.game_results {
        if play.red > RED_LIMIT || play.green > GREEN_LIMIT || play.blue > BLUE_LIMIT {
            return false;
        }
    }

    true
}

fn part1(data: Vec<&str>) {
    let part1_answer: usize = data
        .into_iter()
        .map(get_game_data)
        .filter(is_game_possible)
        .map(|game| game.id)
        .sum();

    println!("Part 1 answer: {part1_answer}");
}

fn get_game_power(game: &Game) -> usize {
    let max_red = game.game_results.iter().map(|hand| hand.red).max().unwrap();
    let max_green = game
        .game_results
        .iter()
        .map(|hand| hand.green)
        .max()
        .unwrap();
    let max_blue = game
        .game_results
        .iter()
        .map(|hand| hand.blue)
        .max()
        .unwrap();

    max_red * max_blue * max_green
}

fn part2(data: Vec<&str>) {
    let game_data: Vec<Game> = data.into_iter().map(get_game_data).collect();

    let part2_answer: usize = game_data.iter().map(get_game_power).sum();

    println!("Part 2 answer: {part2_answer}");
}

fn main() {
    // importing data
    let data = fs::read_to_string("data.txt").unwrap();
    let lines: Vec<&str> = data.split('\n').collect();

    // part 1
    part1(lines.clone());

    // part 2
    part2(lines.clone());
}

#[cfg(test)]
mod tests {
    use crate::get_game_data;
    use crate::get_game_power;
    use crate::is_game_possible;

    #[test]
    fn part1_examples() {
        let game1 = get_game_data("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        assert!(is_game_possible(&game1));
        assert_eq!(game1.id, 1);

        let game2 =
            get_game_data("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue");

        assert!(is_game_possible(&game2));
        assert_eq!(game2.id, 2);

        let game3 = get_game_data(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        );

        assert!(!is_game_possible(&game3));
        assert_eq!(game3.id, 3);

        let game4 = get_game_data(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        );

        assert!(!is_game_possible(&game4));
        assert_eq!(game4.id, 4);

        let game5 = get_game_data("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");

        assert!(is_game_possible(&game5));
        assert_eq!(game5.id, 5);
    }

    #[test]
    fn part2_examples() {
        let game1 = get_game_data("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        assert_eq!(get_game_power(&game1), 48);
    }
}
