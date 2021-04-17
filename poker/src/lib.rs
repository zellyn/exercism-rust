#![feature(map_into_keys_values)]

use std::hash::Hash;
use std::str::FromStr;
use std::{collections::HashMap, convert::TryInto};

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let parsed: Vec<Hand> = hands.iter().map(|s| s.parse::<Hand>().unwrap()).collect();
    let max = parsed.iter().map(Hand::rank).max();
    max.map(|max| {
        hands
            .iter()
            .zip(parsed.iter())
            .filter(|(_, parsed)| parsed.rank() == max)
            .map(|(&hand, _)| hand)
            .collect()
    })
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Copy, Clone)]
enum CardRank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for CardRank {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use CardRank::*;
        match value {
            "2" => Ok(Two),
            "3" => Ok(Three),
            "4" => Ok(Four),
            "5" => Ok(Five),
            "6" => Ok(Six),
            "7" => Ok(Seven),
            "8" => Ok(Eight),
            "9" => Ok(Nine),
            "10" => Ok(Ten),
            "J" => Ok(Jack),
            "Q" => Ok(Queen),
            "K" => Ok(King),
            "A" => Ok(Ace),
            _ => Err(format!("Unknown Rank: '{}'", value)),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Copy, Clone)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl FromStr for Suit {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use Suit::*;
        match value {
            "C" => Ok(Clubs),
            "D" => Ok(Diamonds),
            "H" => Ok(Hearts),
            "S" => Ok(Spades),
            _ => Err(format!("Unknown Suit: '{}'", value)),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
struct Card {
    rank: CardRank,
    suit: Suit,
}

impl FromStr for Card {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let l = value.len();
        if !(2..=3).contains(&l) {
            return Err(format!("Weird length card: '{}'", value));
        }

        let r = value[0..l - 1].parse::<CardRank>();
        let s = value[l - 1..l].parse::<Suit>();
        match (r, s) {
            (Ok(rank), Ok(suit)) => Ok(Card { rank, suit }),
            _ => Err(format!("Weird card: '{}'", value)),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct FullHandRank(HandRank, Vec<CardRank>);

struct Hand {
    cards: [Card; 5],
}

impl FromStr for Hand {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<Card>, String>>()
            .and_then(|v| {
                v.try_into()
                    .map_err(|v: Vec<Card>| format!("want length 5; got {}: '{}'", v.len(), value))
            })
            .map(|cards| Hand { cards })
    }
}

impl Hand {
    fn partition_by<T, F>(&self, f: F) -> Vec<Vec<Card>>
    where
        T: Eq + Hash,
        F: Fn(Card) -> T,
    {
        let mut partitions: Vec<Vec<Card>> = self
            .cards
            .iter()
            .fold(HashMap::new(), |mut a, &card| -> HashMap<_, _> {
                a.entry(f(card)).or_insert_with(Vec::new).push(card);
                a
            })
            .into_values()
            .collect();
        partitions.sort_unstable_by_key(|cards| (cards.len(), cards.first().unwrap().rank));
        partitions.reverse();
        partitions
    }

    fn rank(&self) -> FullHandRank {
        let by_suit: Vec<Vec<Card>> = self.partition_by(|c| c.suit);
        let by_rank: Vec<Vec<Card>> = self.partition_by(|c| c.rank);
        let rank_counts: Vec<usize> = by_rank.iter().map(Vec::len).collect();
        let ranks: Vec<CardRank> = by_rank.iter().map(|cs| cs[0].rank).collect();

        let l = by_rank.len();
        let flush = by_suit.len() == 1;
        let r0 = by_rank[0][0].rank;
        let r1 = by_rank[1][0].rank;
        let rlast = by_rank.last().unwrap()[0].rank;

        let straight = match (l, r0, r1, rlast) {
            (5, r0, _, r4) if r0 as usize == r4 as usize + 4 => true,
            (5, CardRank::Ace, CardRank::Five, CardRank::Two) => true,
            _ => false,
        };

        // StraightFlush
        if straight && flush {
            return FullHandRank(
                HandRank::StraightFlush,
                vec![if r0 == CardRank::Ace {
                    CardRank::Five
                } else {
                    r0
                }],
            );
        }
        // FourOfAKind
        if rank_counts[0] == 4 {
            return FullHandRank(HandRank::FourOfAKind, ranks);
        }
        // FullHouse
        if rank_counts[0] == 3 && rank_counts[1] == 2 {
            return FullHandRank(HandRank::FullHouse, ranks);
        }
        // Flush
        if flush {
            return FullHandRank(HandRank::Flush, ranks);
        }
        // Straight
        if straight {
            return FullHandRank(
                HandRank::Straight,
                vec![if r0 == CardRank::Ace {
                    CardRank::Five
                } else {
                    r0
                }],
            );
        }
        // ThreeOfAKind
        if rank_counts[0] == 3 {
            return FullHandRank(HandRank::ThreeOfAKind, ranks);
        }
        // TwoPair
        if rank_counts[0] == 2 && rank_counts[1] == 2 {
            return FullHandRank(HandRank::TwoPair, ranks);
        }
        // OnePair
        if rank_counts[0] == 2 {
            return FullHandRank(HandRank::OnePair, ranks);
        }
        // HighCard
        FullHandRank(HandRank::HighCard, ranks)
    }
}
