use itertools::Itertools;
advent_of_code::solution!(4);

/// Take the input given by AoC
/// and return an iterator overs cards.
/// Each card is a tuple containing two iterators :
///
/// - The first one iterate over the winning numbers
/// - The second one iterate over the other number
fn parse_cards(
    input: &str,
) -> impl Iterator<Item = (impl Iterator<Item = u32>, impl Iterator<Item = u32>)> + '_ {
    input
        .lines()
        .map(|l| l.split(':').nth(1).unwrap()) // remove "Card 1:"
        .map(|l| {
            l.split('|') // split winning nbrs from the others
                .map(str::split_whitespace)
                .map(|numbers| numbers.map(|n| n.parse::<u32>().unwrap()))
                .map(|numbers| numbers.sorted_unstable())
            // sort the numbers isn't mandatory, but it makes the algorithm a little bit faster
        })
        .map(|mut l| (l.next().unwrap(), l.next().unwrap()))
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = parse_cards(input)
        .map(|(winning, guessing)| {
            let guessing = guessing.collect_vec();
            winning.filter(|nbr| guessing.contains(nbr)).count()
        })
        .filter(|nb_winning| nb_winning > &0)
        .map(|nb_winning| 2_u32.pow(nb_winning as u32 - 1))
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    // cards are gonna be iterated over and over
    // It's more efficient to put it all in vectors
    let mut cards = parse_cards(input)
        .map(|(winning, guessing)| {
            let guessing = guessing.collect_vec();
            (1_u32, winning.filter(|nbr| guessing.contains(nbr)).count())
        })
        .collect_vec();
    for idx in 0..cards.len() {
        let (nb_owned, nb_win) = cards[idx];
        cards
            .iter_mut()
            .skip(idx + 1)
            .take(nb_win)
            .for_each(|card| card.0 += nb_owned);
    }
    let res = cards.iter().map(|(nb_cards, _)| nb_cards).sum();
    Some(res)
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
        let example = "\
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n\
        ";
        assert_eq!(part_two(example), Some(30));
    }
}
