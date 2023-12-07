use itertools::Itertools;
use std::cmp::Ordering;
advent_of_code::solution!(7);

/// Trick using the Rust type system to
/// rewrite the parts that have to be rewritten
/// because of the modification of the rules between part 1 and 2
/// see : https://www.reddit.com/r/rust/comments/18c6i19/comment/kc8nwn2/?utm_source=share&utm_medium=web2x&context=3
struct PartOne<'a>(&'a str);
struct PartTwo<'a>(&'a str);

struct PartOneCards<'a>(&'a Cards);
struct PartTwoCards<'a>(&'a Cards);

const PART_ONE_JOCKER: u8 = 11u8;
const PART_TWO_JOCKER: u8 = 1u8;

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

impl<'a> From<PartOne<'a>> for Cards {
    fn from(PartOne(s): PartOne<'a>) -> Self {
        let res = s
            .chars()
            .map(|c| match c {
                'A' => 14u8,
                'K' => 13u8,
                'Q' => 12u8,
                'J' => PART_ONE_JOCKER,
                'T' => 10u8,
                i => i.to_digit(10).unwrap() as u8,
            })
            .fold(0u32, |acc, n| (acc << 4) | n as u32);
        Self(res)
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

#[derive(Debug, Eq)]
struct Hand {
    cards: Cards,
    strength: Strength,
    bid: u16,
}

impl<'a> From<PartOne<'a>> for Hand {
    /// Here, s is a line from the AoC input
    fn from(PartOne(s): PartOne<'a>) -> Self {
        let mut line = s.split(' ');
        let cards = Cards::from(PartOne(line.next().unwrap()));
        let strength = Strength::from(PartOneCards(&cards));
        Self {
            cards,
            strength,
            bid: line.next().unwrap().parse().unwrap(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.strength == other.strength {
            // if they have the same strength,
            // Compare them by the cards they contain
            self.cards.cmp(&other.cards)
        } else {
            self.strength.cmp(&other.strength)
        }
    }
}
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Strength(u8);

impl<'a> From<PartOneCards<'a>> for Strength {
    fn from(PartOneCards(cards): PartOneCards<'a>) -> Self {
        // bit packed counts
        // 0b00000101 => high card
        // 0b00001011 => one pair
        // 0b00010001 => two pairs
        // 0b00100010 => three of a kind
        // 0b00101000 => full house
        // 0b01000001 => four of a kind
        // 0b10000000 => five of a kind
        let counts = cards.iter().unique().fold(0u8, |counts, i| {
            let count = cards.iter().filter(|&card| card == i).count();
            match count {
                1 => counts + 1,
                2 => counts + (1 << 3),
                i => counts | 1 << (i + 2),
            }
        });
        Self(counts)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|line| Hand::from(PartOne(line)))
        .collect_vec();
    hands.sort();
    let res = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid as u32 * (rank as u32 + 1))
        .sum();
    Some(res)
}

impl<'a> From<PartTwoCards<'a>> for Strength {
    fn from(PartTwoCards(cards): PartTwoCards<'a>) -> Self {
        // bit packed counts
        // 0b00000101 => high card
        // 0b00001011 => one pair
        // 0b00010001 => two pairs
        // 0b00100010 => three of a kind
        // 0b00101000 => full house
        // 0b01000001 => four of a kind
        // 0b10000000 => five of a kind
        let nb_jokers = cards.iter().filter(|&c| c == PART_TWO_JOCKER).count();
        let counts = cards
            .iter()
            .filter(|&c| c != PART_TWO_JOCKER)
            .unique()
            .fold(0u8, |counts, i| {
                let count = cards.iter().filter(|&card| card == i).count();
                match count {
                    1 => counts + 1,
                    2 => counts + (1 << 3),
                    i => counts | 1 << (i + 2),
                }
            });
        if nb_jokers > 0 {
            // There must be a proper algorithm
            // But as there are few values, let's just bruteforce
            Self(match counts {
                // all identical except jokers
                0b0 | 0b01000000 | 0b00100000 | 0b00001000 | 0b00000001 => 0b10000000,

                // all different except jokers
                0b00000010 => 0b01000001, // two cards + three jokers => four of a kind
                0b00000011 => 0b00100010, // three cards + two jokers => three of a kind
                0b00000100 => 0b00001011, // four cards + 1 joker => 1 pair

                // three cards and one different + 1 joker => four of a kind
                0b00100001 => 0b01000001,

                // Two pairs and 1 joker => full house
                0b0010000 => 0b00101000,

                // One pair and two different + 1 joker => three of a kind
                0b00001010 => 0b00100010,

                // One pair, one different + 2 jokers => four of a kind
                0b00001001 => 0b01000001,

                _ => {
                    panic!("This shouldn't happen")
                }
            })
        } else {
            Self(counts)
        }
    }
}

impl<'a> From<PartTwo<'a>> for Cards {
    fn from(PartTwo(s): PartTwo<'a>) -> Self {
        let res = s
            .chars()
            .map(|c| match c {
                'A' => 14u8,
                'K' => 13u8,
                'Q' => 12u8,
                'J' => 1u8,
                'T' => 10u8,
                i => i.to_digit(10).unwrap() as u8,
            })
            .fold(0u32, |acc, n| (acc << 4) | n as u32);
        Self(res)
    }
}

impl<'a> From<PartTwo<'a>> for Hand {
    /// Here, s is a line from the AoC input
    fn from(PartTwo(s): PartTwo<'a>) -> Self {
        let mut line = s.split(' ');
        let cards = Cards::from(PartTwo(line.next().unwrap()));
        let strength = Strength::from(PartTwoCards(&cards));
        Self {
            cards,
            strength,
            bid: line.next().unwrap().parse().unwrap(),
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = input
        .lines()
        .map(|line| Hand::from(PartTwo(line)))
        .collect_vec();
    hands.sort();
    let res = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid as u32 * (rank as u32 + 1))
        .sum();
    Some(res)
}

// answer : 248909434
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
        assert_eq!(result, Some(5905));
    }
}
