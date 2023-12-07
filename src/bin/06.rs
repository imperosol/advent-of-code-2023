advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    duration: f64,
    distance: f64,
}

impl Race {
    /// Return the number of boats that can beat the record,
    /// considering the duration and the record distance
    /// of this race
    fn result(&self) -> u32 {
        let delta = self.duration.powi(2) - 4f64 * self.distance;
        let delta_sqrt = delta.sqrt();
        let bottom_bound = (self.duration - delta_sqrt) / 2f64;
        let upper_bound = (self.duration + delta_sqrt) / 2f64;
        upper_bound.ceil() as u32 - bottom_bound as u32 - 1
    }
}

fn parse_input(s: &str) -> impl Iterator<Item = Race> + '_ {
    let mut lines = s
        .lines()
        .map(str::split_whitespace)
        .map(|numbers| numbers.skip(1).map(|i| i.parse().unwrap()));
    let durations = lines.next().unwrap();
    let distances = lines.next().unwrap();
    durations
        .zip(distances)
        .map(|(duration, distance)| Race { duration, distance })
}

pub fn part_one(input: &str) -> Option<u32> {
    let races = parse_input(input);
    Some(races.map(|race| race.result()).product::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    // The input parser written for part 1
    // does a whitespaces split then remove the first part (row title).
    // This removes all spaces then add back a space after the semicolon
    // to make the same parser usable for part 2
    let input = input.replace(' ', "").replace(':', ": ");
    part_one(input.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
