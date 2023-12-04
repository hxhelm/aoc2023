use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0, multispace1};
use nom::combinator::map_res;
use nom::error::Error;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::{Finish, IResult};
use std::cmp::max;
use std::str::FromStr;
advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    selected_numbers: Vec<u32>,
}

impl FromStr for Card {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_card(s).finish() {
            Ok((_, card)) => Ok(card),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl Card {
    fn get_points(&self) -> u32 {
        self.selected_numbers
            .iter()
            .fold(0, |acc, &selected_number| {
                if self.winning_numbers.contains(&selected_number) {
                    max(acc * 2, 1)
                } else {
                    acc
                }
            })
    }

    fn get_matching_numbers_count(&self) -> usize {
        self.selected_numbers
            .iter()
            .filter(|&selected_number| self.winning_numbers.contains(selected_number))
            .count()
    }
}

fn parse_card_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    // remove any leading whitespace
    let (input, _) = multispace0(input)?;
    separated_list0(multispace1, map_res(digit1, str::parse))(input)
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, card_id) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;

    let (input, (winning_numbers, selected_numbers)) =
        separated_pair(parse_card_numbers, tag(" | "), parse_card_numbers)(input)?;

    Ok((
        input,
        Card {
            id: card_id.parse().unwrap(),
            winning_numbers,
            selected_numbers,
        },
    ))
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = input
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .collect::<Vec<Card>>();

    let sum = cards.iter().fold(0, |acc, card| card.get_points() + acc);

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .collect::<Vec<Card>>();

    let mut scratch_cards = Vec::new();
    for (card_number, card) in cards.iter().enumerate() {
        if scratch_cards.get(card_number).is_none() {
            scratch_cards.push(1);
        } else {
            scratch_cards[card_number] += 1;
        }

        let matching_numbers_count = card.get_matching_numbers_count();

        if matching_numbers_count == 0 {
            continue;
        }

        let current_card_instances = scratch_cards[card_number];

        for i in 1..=matching_numbers_count {
            if scratch_cards.len() <= card_number + i {
                scratch_cards.push(current_card_instances);
            } else {
                scratch_cards[card_number + i] += current_card_instances;
            }
        }
    }

    let sum = scratch_cards.iter().sum::<u32>();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
