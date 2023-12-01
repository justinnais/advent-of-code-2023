advent_of_code::solution!(1);

enum Direction {
    Forward,
    Reverse,
}

fn get_word_value(strings: [&str; 9], word: &str) -> Option<u32> {
    strings
        .iter()
        .position(|&s| word.contains(s))
        .map(|i| i as u32 + 1)
}

fn sliding_window(line: &str, direction: Direction) -> u32 {
    let strs = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let max_len = strs.iter().map(|s| s.len()).max().unwrap();
    let mut num = 0;
    let mut start = 0;
    let mut end = 0;
    match direction {
        Direction::Forward => {
            loop {
                let word = &line[start..=end];
                let last_char = word.chars().last().unwrap();
                if let Some(i) = get_word_value(strs, word) {
                    num = i;
                } else if last_char.is_ascii_digit() {
                    num = last_char.to_digit(10).unwrap();
                }

                if (end - start) > max_len {
                    start += 1;
                }
                end += 1;

                if end == line.len() || num != 0 {
                    break;
                }
            }
            num
        }
        Direction::Reverse => {
            start = line.len() - 1;
            end = line.len() - 1;
            loop {
                let word = &line[start..=end];
                let first_char = word.chars().next().unwrap();
                if let Some(i) = get_word_value(strs, word) {
                    num = i;
                } else if first_char.is_ascii_digit() {
                    num = first_char.to_digit(10).unwrap();
                }

                if (end - start) > max_len {
                    end -= 1;
                }

                if start == 0 || num != 0 {
                    break;
                }
                start -= 1;
            }
            num
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            format!(
                "{}{}",
                line.chars()
                    .find(|c| c.is_ascii_digit())
                    .expect("must return a digit"),
                line.chars()
                    .rev()
                    .find(|c| c.is_ascii_digit())
                    .expect("must return a digit")
            )
            .parse::<u32>()
            .ok()
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            format!(
                "{}{}",
                sliding_window(line, Direction::Forward),
                sliding_window(line, Direction::Reverse)
            )
            .parse::<u32>()
            .ok()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            result,
            Some(
                281 + 59
                    + 21
                    + 83
                    + 11
                    + 11
                    + 22
                    + 22
                    + 33
                    + 33
                    + 44
                    + 44
                    + 55
                    + 55
                    + 66
                    + 66
                    + 77
                    + 77
                    + 88
                    + 88
                    + 99
                    + 99
            )
        );
    }
}
