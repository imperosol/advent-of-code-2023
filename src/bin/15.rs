use crate::Operation::{Insert, Remove};
use itertools::Itertools;
use std::fmt::Error;
advent_of_code::solution!(15);

fn hash(val: &str) -> u32 {
    val.as_bytes()
        .iter()
        .fold(0, |acc, i| ((acc + *i as u32) * 17) % 256)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.split(',').map(hash).sum())
}

#[derive(Debug)]
struct Len<'a> {
    hash: &'a str,
    length: u32,
}

enum Operation<'a> {
    Remove(&'a str),
    Insert(Len<'a>),
}

impl<'a> TryFrom<&'a str> for Operation<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        if let Some(stripped) = s.strip_suffix('-') {
            Ok(Remove(stripped))
        } else if s.contains('=') {
            let mut split = s.splitn(2, '=');
            let hash = split.next().unwrap();
            let length = split.next().unwrap().parse()?;
            Ok(Insert(Len { hash, length }))
        } else {
            Err(Error.into())
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut vecs = (0..256).map(|_| Vec::new()).collect_vec();
    input
        .split(',')
        .for_each(|part| match Operation::try_from(part) {
            Ok(Remove(s)) => {
                let v = vecs.get_mut(hash(s) as usize).unwrap();
                if let Some(ind) = v.iter().position(|l: &Len| l.hash == s) {
                    v.remove(ind);
                }
            }
            Ok(Insert(len)) => {
                let v = vecs.get_mut(hash(len.hash) as usize).unwrap();
                if let Some(elem) = v.iter_mut().find(|l| l.hash == len.hash) {
                    elem.length = len.length
                } else {
                    v.push(len)
                }
            }
            _ => unreachable!(),
        });
    let res = vecs
        .iter()
        .enumerate()
        .flat_map(|(ind, line)| {
            line.iter()
                .enumerate()
                .map(move |(slot, elem)| (ind as u32 + 1) * (slot as u32 + 1) * elem.length)
        })
        .sum();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
