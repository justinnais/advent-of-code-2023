use std::cmp;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line
                .split_once(':')
                .map(|(_, numbers)| {
                    let foo = numbers.split_once('|').unwrap();
                    let left = foo
                        .0
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
                    let right = foo
                        .1
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
                    (left, right)
                })
                .unwrap();

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
    // e.g. won_cards[0] = 10 means that the first line won a total of 10 cards after recursion
    let mut cache: Vec<u32> = Vec::new();
    // cache.resize(input.lines().count(), 0);
    Some(run(input, 0, input.lines().count() - 1, &mut cache))
}

fn run(input: &str, start: usize, end: usize, cache: &mut Vec<u32>) -> u32 {
    let mut count: u32 = 0;
    // for from start to end of input
    for (i, line) in input.lines().enumerate() {
        if i < start {
            continue;
        }
        if i > end || end > input.lines().count() - 1 {
            break;
        }
        count += 1;

        if let Some(value) = cache.get(i) {
            println!("Line {} already calculated as {}", i + 1, value);
            count += value;
            continue;
        }
        cache.push(0);

        println!("Line {}", i + 1);

        let (left, right) = parse_line(&line);
        let mut matches: usize = 0;
        for value in right {
            if left.contains(&value) {
                matches += 1;
            }
        }
        println!("Found {} matches", matches);
        // TODO fix out of bounds
        if matches == 0 {
            continue;
        }
        println!("Copying cards {} to {}", i + 2, i + matches + 1);
        println!(
            "Lines: {:?}",
            &input.lines().collect::<Vec<&str>>()[i + 1..=i + matches]
        );
        println!("\n");
        // let copies = &input.lines().collect::<Vec<&str>>()[i + 1..=i + matches];

        let value = run(input, i + 1, i + matches, cache);
        cache[i] = value;
        count += value;
        println!("Cache: {:?}", cache);
    }
    count
}

fn parse_line(line: &str) -> (Vec<u32>, Vec<u32>) {
    let (left, right) = line
        .split_once(':')
        .map(|(_, numbers)| {
            let foo = numbers.split_once('|').unwrap();
            let left = foo
                .0
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let right = foo
                .1
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (left, right)
        })
        .unwrap();
    (left, right)
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
