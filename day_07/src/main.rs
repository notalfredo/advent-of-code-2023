use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jocker = 11,
    Tekfur = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    One = 1,
    Null = 0,
}

impl Card {
    fn new(card: char) -> Card {
        match card {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jocker,
            'T' => Card::Tekfur,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            '1' => Card::One,
            _ => panic!("DUDE WTF YOU GOT A NOT KNOWN CARD"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn determine_hand_type(given_hand: &[Card; 5]) -> HandType {
        let mut curr_hand: Vec<(Card, u32)> = Vec::new();

        given_hand.iter().for_each(|card| {
            let found_card = curr_hand
                .iter_mut()
                .find(|(curr_hand_card, _)| *curr_hand_card == *card);

            match found_card {
                Some(card_found) => {
                    card_found.1 += 1;
                }
                None => {
                    curr_hand.push((*card, 1));
                }
            }
        });

        curr_hand.sort_by(|(_, count_a), (_, count_b)| count_b.partial_cmp(count_a).unwrap());

        if curr_hand.iter().map(|(_, count)| count).sum::<u32>() != 5 {
            panic!(
                "Got a curr_hand of length {:}",
                curr_hand.iter().map(|(_, count)| count).sum::<u32>()
            );
        }

        if curr_hand.len() == 1 && curr_hand[0].1 == 5 {
            HandType::FiveOfAKind
        } else if curr_hand.len() == 2 && curr_hand[0].1 == 4 && curr_hand[1].1 == 1 {
            HandType::FourOfAKind
        } else if curr_hand.len() == 2 && curr_hand[0].1 == 3 && curr_hand[1].1 == 2 {
            HandType::FullHouse
        } else if curr_hand.len() == 3
            && curr_hand[0].1 == 3
            && curr_hand[1].1 == 1
            && curr_hand[2].1 == 1
        {
            HandType::ThreeOfAKind
        } else if curr_hand.len() == 3
            && curr_hand[0].1 == 2
            && curr_hand[1].1 == 2
            && curr_hand[2].1 == 1
        {
            HandType::TwoPair
        } else if curr_hand.len() == 4
            && curr_hand[0].1 == 2
            && curr_hand[1].1 == 1
            && curr_hand[2].1 == 1
            && curr_hand[3].1 == 1
        {
            HandType::OnePair
        } else if curr_hand.len() == 5 {
            HandType::HighCard
        } else {
            panic!("DUDE YOU GOT A UNKOWN HAND TYPE WTF");
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct Hand {
    given_hand: [Card; 5],
    bid: u32,
    hand_type: HandType,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        for (card_left, card_right) in self.given_hand.iter().zip(other.given_hand.iter()) {
            if *card_left < *card_right {
                return Some(Ordering::Less);
            } else if *card_left > *card_right {
                return Some(Ordering::Greater);
            }
        }
        return Some(Ordering::Equal);
    }
}

impl Hand {
    fn new(passed_cards: &str, passed_bid: &str) -> Hand {
        let mut given_hand: [Card; 5] =
            [Card::Null, Card::Null, Card::Null, Card::Null, Card::Null];
        let bid = passed_bid.trim().parse::<u32>().unwrap();
        for (index, char) in passed_cards.chars().enumerate() {
            given_hand[index] = Card::new(char);
        }

        let hand_type = HandType::determine_hand_type(&given_hand);
        Hand {
            given_hand,
            bid,
            hand_type,
        }
    }

    fn dump(&self) {
        println!("{:?}, {:}, {:?}", self.given_hand, self.bid, self.hand_type);
    }
}

struct CamelCards {
    hands: Vec<Hand>,
}

impl CamelCards {
    fn new(file: &str) -> CamelCards {
        let hands = file
            .lines()
            .map(|line| {
                let (cards, bid) = line.split_once(" ").unwrap();
                Hand::new(cards, bid)
            })
            .collect::<Vec<Hand>>();

        CamelCards { hands }
    }

    fn rank_cards_q1(&mut self) -> u32 {
        let mut high_cards = self
            .hands
            .iter()
            .filter(|hand| hand.hand_type == HandType::HighCard)
            .map(|hand| *hand)
            .collect::<Vec<Hand>>();
        let mut one_pairs = self
            .hands
            .iter()
            .filter(|hand| hand.hand_type == HandType::OnePair)
            .map(|hand| *hand)
            .collect::<Vec<Hand>>();
        let mut two_pairs = self
            .hands
            .iter()
            .filter(|hand| hand.hand_type == HandType::TwoPair)
            .map(|hand| *hand)
            .collect::<Vec<Hand>>();
        let mut three_of_one_kind = self
            .hands
            .iter()
            .filter(|hand| hand.hand_type == HandType::ThreeOfAKind)
            .map(|hand| *hand)
            .collect::<Vec<Hand>>();
        let mut full_house = self
            .hands
            .iter()
            .filter(|hand| hand.hand_type == HandType::FullHouse)
            .map(|hand| *hand)
            .collect::<Vec<Hand>>();
        let mut four_of_a_kind = self
            .hands
            .iter()
            .filter(|hand| hand.hand_type == HandType::FourOfAKind)
            .map(|hand| *hand)
            .collect::<Vec<Hand>>();
        let mut five_of_a_kind = self
            .hands
            .iter()
            .filter(|hand| hand.hand_type == HandType::FiveOfAKind)
            .map(|hand| *hand)
            .collect::<Vec<Hand>>();

        high_cards.sort_by(|cards_a, cards_b| cards_a.partial_cmp(cards_b).unwrap());
        one_pairs.sort_by(|cards_a, cards_b| cards_a.partial_cmp(cards_b).unwrap());
        two_pairs.sort_by(|cards_a, cards_b| cards_a.partial_cmp(cards_b).unwrap());
        three_of_one_kind.sort_by(|cards_a, cards_b| cards_a.partial_cmp(cards_b).unwrap());
        full_house.sort_by(|cards_a, cards_b| cards_a.partial_cmp(cards_b).unwrap());
        four_of_a_kind.sort_by(|cards_a, cards_b| cards_a.partial_cmp(cards_b).unwrap());
        five_of_a_kind.sort_by(|cards_a, cards_b| cards_a.partial_cmp(cards_b).unwrap());


        let mut new_hands: Vec<Hand> = Vec::new();
        new_hands.append(&mut high_cards) ;
        new_hands.append(&mut one_pairs);
        new_hands.append(&mut two_pairs);
        new_hands.append(&mut three_of_one_kind);
        new_hands.append(&mut full_house);
        new_hands.append(&mut four_of_a_kind);
        new_hands.append(&mut five_of_a_kind);

        self.hands = new_hands;

        let mut sum: u32 = 0;
        for (index, hand) in self.hands.iter().enumerate() {
            sum += ((index + 1) as u32) * hand.bid;
        }
        sum
    }

    fn dump(&self) {
        for hand in &self.hands {
            hand.dump();
        }
    }
}

fn main() {
    let file = include_str!("../input/input_file.txt");
    let mut my_camel_cards = CamelCards::new(file);
    println!("Question 1: {:}", my_camel_cards.rank_cards_q1());
}
