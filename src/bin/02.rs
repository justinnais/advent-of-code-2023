use std::collections::HashMap;

advent_of_code::solution!(2);

#[derive(Debug, Clone, Copy)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}
impl Round {
    fn from(str: &str) -> Self {
        let mut map: HashMap<&str, u32> = HashMap::new();
        for part in str.split(", ") {
            let mut iter = part.split(' ');
            let count = iter.next().unwrap().parse::<u32>().unwrap();
            let color = iter.next().unwrap();
            map.insert(color, count);
        }
        Self {
            red: *map.get("red").unwrap_or(&0),
            green: *map.get("green").unwrap_or(&0),
            blue: *map.get("blue").unwrap_or(&0),
        }
    }

    fn max(&self, other: &Self) -> Self {
        Self {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn greater_than(&self, other: Self) -> bool {
        self.red > other.red || self.green > other.green || self.blue > other.blue
    }

    fn pow(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

const MAX_CUBES: Round = Round {
    red: 12,
    green: 13,
    blue: 14,
};

fn parse_line(line: &str) -> Vec<Round> {
    line.split(": ")
        .nth(1)
        .unwrap()
        .split("; ")
        .map(Round::from)
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let rounds = parse_line(line);
            let mut valid = true;
            for round in rounds {
                if round.greater_than(MAX_CUBES) {
                    valid = false;
                    break;
                }
            }
            match valid {
                true => Some(i as u32 + 1),
                false => Some(0),
            }
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(parse_line)
        .map(|rounds| {
            return Some(
                rounds
                    .iter()
                    .fold(rounds[0], |acc, round| acc.max(round))
                    .pow(),
            );
        })
        .sum()
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
