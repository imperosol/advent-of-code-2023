advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let res = input
        .lines()
        .map(|l| {
            let first = l.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last = l.chars().rfind(|c| c.is_ascii_digit()).unwrap();
            first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
        })
        .sum();
    Some(res)
}

const DIGIT_STR: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight",
    "nine", // "1", "2", "3", "4", "5", "6", "7", "8", "9"
];

fn str_to_digit(m: &str) -> Option<u32> {
    DIGIT_STR
        .iter()
        .position(|&i| i == m)
        .map(|pos| (pos + 1) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // words can overlap (eg "eightree")
    // The regex crate doesn't support backtracking by design,
    // thus it can't be used for this problem
    let res = input
        .lines()
        .map(|l| {
            let first = DIGIT_STR
                .iter()
                .filter_map(|s| l.find(s).map(|ind| (str_to_digit(s).unwrap(), ind)))
                .chain(
                    ('0'..='9').filter_map(|c| l.find(c).map(|ind| (c.to_digit(10).unwrap(), ind))),
                )
                .min_by_key(|i| i.1)
                .map(|i| i.0);
            let last = DIGIT_STR
                .iter()
                .filter_map(|s| l.rfind(s).map(|ind| (str_to_digit(s).unwrap(), ind)))
                .chain(
                    ('0'..='9')
                        .filter_map(|c| l.rfind(c).map(|ind| (c.to_digit(10).unwrap(), ind))),
                )
                .max_by_key(|i| i.1)
                .map(|i| i.0);
            first.unwrap() * 10 + last.unwrap()
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
