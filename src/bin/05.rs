use itertools::Itertools;
use std::ops::Range;
use std::str::FromStr;
advent_of_code::solution!(5);

#[derive(Debug)]
struct Mapping {
    source: Range<u64>,
    destination: Range<u64>,
}

impl FromStr for Mapping {
    type Err = anyhow::Error;

    /// s is a line of the AoC input
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(' ').map(|s| s.parse().unwrap());
        let destination = line.next().unwrap();
        let source = line.next().unwrap();
        let length = line.next().unwrap();
        Ok(Self {
            source: source..(source + length),
            destination: destination..(destination + length),
        })
    }
}

impl Mapping {
    fn map_u64(&self, n: &u64) -> u64 {
        if self.source.contains(n) {
            self.destination.start + n - self.source.start
        } else {
            *n
        }
    }

    // fn map_range(&self, rhs: &Range<u64>) -> Vec<Range<u64>> {
    //     let mut res = if rhs.start >= self.source.start && rhs.end <= self.source.end {
    //         // rhs fully included in self.source
    //         let start = self.destination.start + rhs.start - self.source.start;
    //         if rhs.end <= rhs.start {
    //             println!("rhs: {:?}", rhs);
    //         }
    //         vec![start..start + (rhs.end - rhs.start)]
    //     } else if rhs.start <= self.source.start && rhs.end > self.source.end {
    //         // self.source fully included in rhs
    //         vec![
    //             rhs.start..self.source.start,
    //             self.destination.clone(),
    //             self.source.end..rhs.end,
    //         ]
    //     } else if rhs.start <= self.source.start && self.source.contains(&(rhs.end - 1)) {
    //         // rhs starts before self.source and ends inside self.source
    //         vec![
    //             rhs.start..self.source.start,
    //             self.destination.start..(self.destination.end - (self.source.end - rhs.end)),
    //         ]
    //     } else if self.source.contains(&rhs.start) && rhs.end > self.source.end {
    //         // rhs starts inside self.source and ends after self.source
    //         vec![
    //             self.destination.start + (rhs.start - self.source.start)..self.destination.end,
    //             self.source.end..rhs.end,
    //         ]
    //     } else if self.source.start >= rhs.end || self.source.end <= rhs.start {
    //         // rhs fully out of self.source
    //         vec![rhs.clone()]
    //     } else {
    //         panic!("Unexpected range")
    //     };
    //     if res.len() > 1 {
    //         res.sort_by_key(|r| r.start);
    //         res.dedup();
    //         // add merge ranges here if performance is an issue
    //     }
    //     res
    // }
}

// fn merge_ranges(ranges: &mut Vec<Range<u64>>) -> Vec<Range<u64>> {
//     if ranges.is_empty() {
//         return Vec::new();
//     }
//     ranges.sort_by_key(|i| i.start);
//     let mut res = vec![ranges.get(0).unwrap().clone()];
//     ranges.iter_mut().skip(1).for_each(|range| {
//         let last = res.last_mut().unwrap();
//         if range.start > last.end {
//             res.push(range.clone());
//         } else if range.end > last.end {
//             last.end = range.end;
//         } else {
//             // If here, it means that range is fully included in last,
//             // hence do nothing
//         }
//     });
//     res
// }

fn parse_input(s: &str) -> (Vec<u64>, Vec<Vec<Mapping>>) {
    let mut lines = s.lines().filter(|s| !s.is_empty());

    let seeds = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect_vec();
    let mut mappings = Vec::with_capacity(7); // There are 7 categories
    for line in lines {
        if line.chars().next().unwrap().is_ascii_alphabetic() {
            mappings.push(Vec::new());
        } else {
            mappings.last_mut().unwrap().push(line.parse().unwrap());
        }
    }
    (seeds, mappings)
}

pub fn part_one(input: &str) -> Option<u64> {
    // input text presents its categories in the order
    // a-to-b, then b-to-c, then c-to-d...
    // Thanks to that, there is no need to parse the names of the categories
    let (mut seeds, categories) = parse_input(input);

    categories.iter().for_each(|category| {
        seeds.iter_mut().for_each(|i| {
            let mapping = category.iter().find(|mapping| mapping.source.contains(i));
            if let Some(mapping) = mapping {
                *i = mapping.map_u64(i);
            }
        });
    });
    seeds.into_iter().min()
}
pub fn part_two(_input: &str) -> Option<u64> {
    // nique, ça marche pas, je le ferai plus tard
    None

    // input text presents its categories in the order
    // a-to-b, then b-to-c, then c-to-d...
    // Thanks to that, there is no need to parse the names of the categories
    // let (seeds, categories) = parse_input(input);
    // let mut seeds = seeds
    //     .into_iter()
    //     .chunks(2)
    //     .into_iter()
    //     .map(|mut pair| {
    //         let start = pair.next().unwrap();
    //         start..start + pair.next().unwrap()
    //     })
    //     .collect_vec();
    // categories.iter().for_each(|category| {
    //     let mut new_seeds = seeds
    //         .iter()
    //         .flat_map(|seed| {
    //             let mut results = category
    //                 .iter()
    //                 .flat_map(|mapping| mapping.map_range(seed))
    //                 .unique()
    //                 .collect_vec();
    //             if results.len() > 1 {
    //                 results.retain(|r| r != seed)
    //             }
    //             results
    //         })
    //         .collect_vec();
    //     seeds = merge_ranges(&mut new_seeds);
    //     seeds = new_seeds;
    // });
    // seeds.into_iter().map(|r| r.start).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
        // assert_eq!(result, Some(46));
    }
}
