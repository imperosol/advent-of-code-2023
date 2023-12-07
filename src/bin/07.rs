use crate::Strength::*;
use itertools::Itertools;
use std::cmp::Ordering;
use std::str::FromStr;
advent_of_code::solution!(7);

/// Bit packed cards
/// Each card is contained on 4 bits (total : 2 and a half bytes)
#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
struct Cards(u32);

impl Cards {
    fn iter(&self) -> CardIterator {
        // The card iterator evaluates the four
        // weakest bits then move the others to the right.
        // As the first cards are stored in the strongest bits,
        // We must invert the groups of 4 bits
        let iterator = (self.0 & 0xF0000) >> 16
            | (self.0 & 0xF000) >> 8
            | (self.0 & 0xF00)
            | (self.0 & 0xF0) << 8
            | (self.0 & 0xF) << 16;
        CardIterator(iterator)
    }
}

impl FromStr for Cards {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s
            .chars()
            .map(|c| match c {
                'A' => 14u8,
                'K' => 13u8,
                'Q' => 12u8,
                'J' => 11u8,
                'T' => 10u8,
                i => i.to_digit(10).unwrap() as u8,
            })
            .fold(0u32, |acc, n| (acc << 4) | n as u32);
        Ok(Self(res))
    }
}

struct CardIterator(u32);

impl Iterator for CardIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            // get the four weak bits
            let res = self.0 & 0b1111;
            // then left shift remaining bits to collect the next
            // card during the next iteration
            self.0 >>= 4;
            Some(res as u8)
        }
    }
}

#[derive(Debug, Eq, Ord)]
struct Hand {
    cards: Cards,
    strength: Strength,
    bid: u16,
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    /// Here, s is a line from the AoC input
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(' ');
        let cards = line.next().unwrap().parse()?;
        let strength = Strength::from(&cards);
        Ok(Self {
            cards,
            strength,
            bid: line.next().unwrap().parse()?,
        })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.strength == other.strength {
            // if they have the same strength,
            // Compare them by the cards they contain
            self.cards.partial_cmp(&other.cards)
        } else {
            self.strength.partial_cmp(&other.strength)
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Strength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&Cards> for Strength {
    fn from(cards: &Cards) -> Self {
        // bit packed counts
        // three weakest bits => number of single cards
        // two following bits => number of pairs
        // one following bit => three cards
        // one following bit => four cards
        // one following bit => five cards
        let counts = cards.iter().unique().fold(0u8, |counts, i| {
            let count = cards.iter().filter(|&card| card == i).count();
            match count {
                1 => counts + 1,
                2 => (((counts >> 3) + 1) << 3) | (counts & 0b111),
                i => counts | 1 << (i + 2),
            }
        });
        match counts {
            0b00000101 => HighCard,
            0b00001011 => OnePair,
            0b00010001 => TwoPair,
            0b00100010 => ThreeOfAKind,
            0b00101000 => FullHouse,
            0b01000001 => FourOfAKind,
            0b10000000 => FiveOfAKind,
            _ => panic!("This shouldn't happen"),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|line| line.parse::<Hand>().unwrap())
        .collect_vec();
    hands.sort();
    let res = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid as u32 * (rank as u32 + 1))
        .sum();
    Some(res)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let input = "\
            32T3K 765\n\
            T55J5 684\n\
            KK677 28\n\
            KTJJT 220\n\
            QQQJA 483\
        ";
        let result = part_two(input);
        assert_eq!(result, None);
    }
}
