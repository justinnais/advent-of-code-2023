advent_of_code::solution!(1);

enum Direction {
    Forward,
    Reverse,
}

struct Window {
    start: usize,
    end: usize,
}
impl Window {
    fn new(index: usize) -> Self {
        Self {
            start: index,
            end: index,
        }
    }
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
    let mut value = 0;
    let Window { mut start, mut end } = match direction {
        Direction::Forward => Window::new(0),
        Direction::Reverse => Window::new(line.len() - 1),
    };

    loop {
        let word = &line[start..=end];
        let char = match direction {
            Direction::Forward => word.chars().last().unwrap(),
            Direction::Reverse => word.chars().next().unwrap(),
        };
        if let Some(i) = get_word_value(strs, word) {
            value = i;
        } else if char.is_ascii_digit() {
            value = char.to_digit(10).unwrap();
        }

        match direction {
            Direction::Forward => {
                if (end - start) > max_len {
                    start += 1;
                }
                if end == line.len() || value != 0 {
                    break;
                }
                end += 1;
            }
            Direction::Reverse => {
                if (end - start) > max_len {
                    end -= 1;
                }
                if start == 0 || value != 0 {
                    break;
                }
                start -= 1;
            }
        }
    }
    value
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
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(1434));
    }
}
