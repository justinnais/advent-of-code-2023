advent_of_code::solution!(7);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
#[derive(Clone)]
enum Card {
    Number(u32),
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Debug)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
impl Type {
    fn from_cards(cards: &Vec<Card>) -> Self {
        let mut counts = [0; 15];
        for card in cards {
            match card {
                Card::Number(n) => counts[*n as usize] += 1,
                Card::Jack => counts[11] += 1,
                Card::Queen => counts[12] += 1,
                Card::King => counts[13] += 1,
                Card::Ace => counts[14] += 1,
            }
        }
        match counts.iter().max().unwrap() {
            5 => Self::FiveOfAKind,
            4 => Self::FourOfAKind,
            3 => {
                if counts.iter().filter(|&&n| n == 2).count() == 1 {
                    Self::FullHouse
                } else {
                    Self::ThreeOfAKind
                }
            }
            2 => {
                if counts.iter().filter(|&&n| n == 2).count() == 2 {
                    Self::TwoPair
                } else {
                    Self::OnePair
                }
            }
            1 => Self::HighCard,
            _ => panic!("Invalid card count"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Hand {
    cards: Vec<Card>,
    hand_type: Type,
}
impl Hand {
    fn new(input: &str) -> Self {
        let cards: Vec<Card> = input
            .chars()
            .map(|char| match char {
                'T' => Card::Number(10),
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => Card::Number(char.to_digit(10).unwrap()),
            })
            .collect();
        let hand_type = Type::from_cards(&cards);

        Self { cards, hand_type }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<(Hand, u32)> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.trim().split_once(" ").unwrap();
            (Hand::new(hand), bid.parse::<u32>().unwrap())
        })
        .collect();
    hands.sort_by(|(hand_a, _), (hand_b, _)| {
        hand_a.cmp(hand_b)
    });
    for (hand, bid) in hands {
        println!("{:?} {}", hand, bid);
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
