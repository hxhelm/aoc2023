use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::error::Error;
use nom::multi::separated_list0;
use nom::{Finish, IResult};
use std::str::FromStr;
advent_of_code::solution!(2);

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (game, _) = tag("Game ")(input)?;
    let (game, game_id) = digit1(game)?;
    let (game, _) = tag(": ")(game)?;

    let (game, rounds) = separated_list0(tag("; "), parse_round)(game)?;

    Ok((
        game,
        Game {
            id: game_id.parse().unwrap(),
            rounds,
        },
    ))
}

impl FromStr for Game {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_game(s).finish() {
            Ok((_, game)) => Ok(game),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl Game {
    fn get_max_cubes(&self) -> (u32, u32, u32) {
        let &min_red = self
            .rounds
            .iter()
            .flat_map(|round| round.red_cube_counts())
            .max()
            .unwrap();

        let &min_green = self
            .rounds
            .iter()
            .flat_map(|round| round.green_cube_counts())
            .max()
            .unwrap();

        let &min_blue = self
            .rounds
            .iter()
            .flat_map(|round| round.blue_cube_counts())
            .max()
            .unwrap();

        (min_red, min_green, min_blue)
    }

    fn is_possible(&self) -> bool {
        let (max_red, max_green, max_blue) = self.get_max_cubes();
        max_red <= MAX_RED && max_green <= MAX_GREEN && max_blue <= MAX_BLUE
    }
}

#[derive(Debug)]
struct Round {
    cubes: Vec<Cube>,
}

impl Round {
    fn red_cube_counts(&self) -> Vec<&u32> {
        self.cubes
            .iter()
            .filter_map(|cube| match cube {
                Cube::Red(count) => Some(count),
                _ => None,
            })
            .collect()
    }

    fn green_cube_counts(&self) -> Vec<&u32> {
        self.cubes
            .iter()
            .filter_map(|cube| match cube {
                Cube::Green(count) => Some(count),
                _ => None,
            })
            .collect()
    }

    fn blue_cube_counts(&self) -> Vec<&u32> {
        self.cubes
            .iter()
            .filter_map(|cube| match cube {
                Cube::Blue(count) => Some(count),
                _ => None,
            })
            .collect()
    }
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (input, cubes) = separated_list0(tag(", "), parse_cube)(input)?;

    Ok((input, Round { cubes }))
}

#[derive(Debug)]
enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn parse_cube(input: &str) -> IResult<&str, Cube> {
    let (input, count) = digit1(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = alpha1(input)?;

    let count = count.parse().unwrap();

    match color {
        "red" => Ok((input, Cube::Red(count))),
        "green" => Ok((input, Cube::Green(count))),
        "blue" => Ok((input, Cube::Blue(count))),
        _ => panic!("Unknown color"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let games = lines
        .map(|line| line.parse::<Game>().unwrap())
        .collect::<Vec<Game>>();

    let sum = games
        .iter()
        .filter(|game| game.is_possible())
        .fold(0, |acc, game| acc + game.id);

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let games = lines
        .map(|line| line.parse::<Game>().unwrap())
        .collect::<Vec<Game>>();

    let powers = games.iter().fold(0, |acc, game| -> u32 {
        let (max_red, max_green, max_blue) = game.get_max_cubes();
        acc + (max_red * max_green * max_blue)
    });

    Some(powers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
