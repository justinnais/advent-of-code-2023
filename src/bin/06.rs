advent_of_code::solution!(6);

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}
impl Race {
    fn margin_of_error(&self) -> u64 {
        let mut left = 0;
        let mut right = self.time;

        fn is_winner(race: &Race, duration: u64) -> bool {
            let distance = duration * (race.time - duration);
            if distance > race.distance {
                return true;
            }
            false
        }

        while left <= right && !is_winner(self, left) {
            left += 1;
        }
        while left <= right && !is_winner(self, right) {
            right -= 1;
        }

        right - left + 1
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut iter = input.lines().map(|line| {
        line.split(":")
            .last()
            .unwrap()
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
    });

    let times = iter.next().unwrap();
    let mut distances = iter.next().unwrap();

    let mut count: u64 = 0;

    for time in times {
        let race = Race {
            time,
            distance: distances.next().unwrap(),
        };

        match count {
            0 => count += race.margin_of_error(),
            _ => count *= race.margin_of_error(),
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut iter = input.lines().map(|line| {
        line.split(":")
            .last()
            .unwrap()
            .replace(" ", "")
            .parse::<u64>()
            .unwrap()
    });

    let race = Race {
        time: iter.next().unwrap(),
        distance: iter.next().unwrap(),
    };

    Some(race.margin_of_error())
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
