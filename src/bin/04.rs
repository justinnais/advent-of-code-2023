advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let (left, right) = parse_line(line);
            let mut points: u32 = 0;

            for value in right {
                if left.contains(&value) {
                    match points {
                        0 => points = 1,
                        _ => points *= 2,
                    }
                }
            }

            points
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    // array of cards won for each index
    // e.g. cache[0] = 10 means that the first line won a total of 10 cards after processing all copies won
    let mut cache: Vec<u32> = Vec::new();
    Some(run(input, 0, input.lines().count() - 1, &mut cache))
}

fn run(input: &str, start: usize, end: usize, cache: &mut Vec<u32>) -> u32 {
    let mut count: u32 = 0;

    for (i, line) in input.lines().enumerate() {
        if i < start {
            continue;
        }
        if i > end || end > input.lines().count() - 1 {
            break;
        }

        count += 1;

        if let Some(value) = cache.get(i) {
            count += value;
            continue;
        }
        cache.push(0);

        let (left, right) = parse_line(line);
        let mut matches: usize = 0;

        for value in right {
            if left.contains(&value) {
                matches += 1;
            }
        }

        if matches == 0 {
            break;
        }

        let value = run(input, i + 1, i + matches, cache);
        cache[i] = value;
        count += value;
    }
    count
}

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let (left, right) = line
        .split_once(':')
        .map(|(_, numbers)| {
            let (left, right) = numbers.split_once('|').unwrap();
            (parse_numbers(left), parse_numbers(right))
        })
        .unwrap();
    (left, right)
}

fn parse_numbers(numbers: &str) -> Vec<u32> {
    numbers
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
