use std::{cmp::Ordering, io::Read};

#[derive(Debug)]
struct CardsBid {
    cards: Vec<u32>,
    bid: u64,
}

//Needed because sorting function now needs to place joker as the weakest card
#[derive(Debug)]
struct CardsBid2 {
    cards: Vec<u32>,
    bid: u64,
}

fn main() {
    let mut file = std::fs::File::open("./input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let lines: Vec<_> = contents.split("\r\n").collect();

    let part_1_res = part_1(&lines);
    println!("part1_res: {part_1_res:?}");
    let part_2_res = part_2(&lines);
    println!("part2_res: {part_2_res:?}");
}

fn part_2(lines: &Vec<&str>) -> u64 {
    let mut result = 0;
    let cards_bids: Vec<CardsBid2> = lines
        .iter()
        .map(|x| {
            let vec = x.split_whitespace().collect::<Vec<_>>();
            let bid = vec[1].parse::<u64>().expect("always number");
            let cards: Vec<_> = vec[0].chars().collect();
            let mut cards_parsed: Vec<u32> = Vec::new();
            for card in cards {
                if card == 'T' {
                    cards_parsed.push(10);
                } else if card == 'J' {
                    cards_parsed.push(11);
                } else if card == 'Q' {
                    cards_parsed.push(12);
                } else if card == 'K' {
                    cards_parsed.push(13);
                } else if card == 'A' {
                    cards_parsed.push(14);
                } else {
                    cards_parsed.push(card.to_digit(10).expect("always number in this step"));
                }
            }
            return CardsBid2 {
                cards: cards_parsed,
                bid,
            };
        })
        .collect();

    let mut five_kind: Vec<&CardsBid2> = Vec::new();
    let mut four_kind: Vec<&CardsBid2> = Vec::new();
    let mut full_house: Vec<&CardsBid2> = Vec::new();
    let mut three_kind: Vec<&CardsBid2> = Vec::new();
    let mut two_pair: Vec<&CardsBid2> = Vec::new();
    let mut one_pair: Vec<&CardsBid2> = Vec::new();
    let mut high_card: Vec<&CardsBid2> = Vec::new();

    for card_bid in &cards_bids {
        let mut cards: Vec<u32> = card_bid.cards.clone();
        cards.sort();
        let card_replaced = cards.clone();
        let mut highest_rank = 0;
        for i in 2..15 {
            let mut cards = card_replaced
                .iter()
                .map(|x| {
                    if x == &11 {
                        return i;
                    } else {
                        return *x;
                    }
                })
                .collect::<Vec<u32>>();
            cards.sort();
            if cards.iter().filter(|n| **n == cards[0]).count() == 5 {
                highest_rank = 10;
                continue;
            } else if cards.iter().filter(|n| **n == cards[0]).count() == 4
                || cards.iter().filter(|n| **n == cards[1]).count() == 4
            {
                if highest_rank < 9 {
                    highest_rank = 9;
                }

                continue;
            } else if cards[0] == cards[1]
                && cards[4] == cards[3]
                && (cards[1] == cards[2] || cards[3] == cards[2])
            {
                if highest_rank < 8 {
                    highest_rank = 8;
                }
                continue;
            } else if (cards[0] == cards[1] && cards[1] == cards[2])
                || (cards[1] == cards[2] && cards[2] == cards[3])
                || (cards[2] == cards[3] && cards[3] == cards[4])
            {
                if highest_rank < 7 {
                    highest_rank = 7;
                }

                continue;
            } else if (cards[0] == cards[1] && (cards[2] == cards[3] || cards[3] == cards[4]))
                || (cards[1] == cards[2] && cards[3] == cards[4])
            {
                if highest_rank < 6 {
                    highest_rank = 6;
                }
                continue;
            } else if cards[0] == cards[1]
                || cards[1] == cards[2]
                || cards[2] == cards[3]
                || cards[3] == cards[4]
            {
                if highest_rank < 5 {
                    highest_rank = 5;
                }
                continue;
            } else {
                if highest_rank < 4 {
                    highest_rank = 4;
                }
            }
        }
        if highest_rank == 10 {
            five_kind.push(card_bid);
        } else if highest_rank == 9 {
            four_kind.push(card_bid);
        } else if highest_rank == 8 {
            full_house.push(card_bid);
        } else if highest_rank == 7 {
            three_kind.push(card_bid);
        } else if highest_rank == 6 {
            two_pair.push(card_bid);
        } else if highest_rank == 5 {
            one_pair.push(card_bid);
        } else {
            high_card.push(card_bid);
        }
    }
    five_kind.sort();
    four_kind.sort();
    full_house.sort();
    three_kind.sort();
    two_pair.sort();
    one_pair.sort();
    high_card.sort();
    five_kind.append(&mut four_kind);
    five_kind.append(&mut full_house);
    five_kind.append(&mut three_kind);
    five_kind.append(&mut two_pair);
    five_kind.append(&mut one_pair);
    five_kind.append(&mut high_card);

    for i in 0..five_kind.len() {
        result += five_kind[five_kind.len() - 1 - i].bid * (i as u64 + 1);
    }

    result
}

fn part_1(lines: &Vec<&str>) -> u64 {
    let mut result = 0;

    let cards_bids: Vec<CardsBid> = lines
        .iter()
        .map(|x| {
            let vec = x.split_whitespace().collect::<Vec<_>>();
            let bid = vec[1].parse::<u64>().expect("always number");
            let cards: Vec<_> = vec[0].chars().collect();
            let mut cards_parsed: Vec<u32> = Vec::new();
            for card in cards {
                if card == 'T' {
                    cards_parsed.push(10);
                } else if card == 'J' {
                    cards_parsed.push(11);
                } else if card == 'Q' {
                    cards_parsed.push(12);
                } else if card == 'K' {
                    cards_parsed.push(13);
                } else if card == 'A' {
                    cards_parsed.push(14);
                } else {
                    cards_parsed.push(card.to_digit(10).expect("always number in this step"));
                }
            }
            return CardsBid {
                cards: cards_parsed,
                bid,
            };
        })
        .collect();
    let mut five_kind: Vec<&CardsBid> = Vec::new();
    let mut four_kind: Vec<&CardsBid> = Vec::new();
    let mut full_house: Vec<&CardsBid> = Vec::new();
    let mut three_kind: Vec<&CardsBid> = Vec::new();
    let mut two_pair: Vec<&CardsBid> = Vec::new();
    let mut one_pair: Vec<&CardsBid> = Vec::new();
    let mut high_card: Vec<&CardsBid> = Vec::new();

    for card_bid in &cards_bids {
        let mut cards: Vec<u32> = card_bid.cards.clone();
        cards.sort();
        if cards.iter().filter(|n| **n == cards[0]).count() == 5 {
            five_kind.push(card_bid);
            continue;
        } else if cards.iter().filter(|n| **n == cards[0]).count() == 4
            || cards.iter().filter(|n| **n == cards[1]).count() == 4
        {
            four_kind.push(card_bid);
            continue;
        } else if cards[0] == cards[1]
            && cards[4] == cards[3]
            && (cards[1] == cards[2] || cards[3] == cards[2])
        {
            full_house.push(card_bid);
            continue;
        } else if (cards[0] == cards[1] && cards[1] == cards[2])
            || (cards[1] == cards[2] && cards[2] == cards[3])
            || (cards[2] == cards[3] && cards[3] == cards[4])
        {
            three_kind.push(card_bid);
            continue;
        } else if (cards[0] == cards[1] && (cards[2] == cards[3] || cards[3] == cards[4]))
            || (cards[1] == cards[2] && cards[3] == cards[4])
        {
            two_pair.push(card_bid);
            continue;
        } else if cards[0] == cards[1]
            || cards[1] == cards[2]
            || cards[2] == cards[3]
            || cards[3] == cards[4]
        {
            one_pair.push(card_bid);
            continue;
        } else {
            high_card.push(card_bid);
        }
    }
    five_kind.sort();
    four_kind.sort();
    full_house.sort();
    three_kind.sort();
    two_pair.sort();
    one_pair.sort();
    high_card.sort();
    five_kind.append(&mut four_kind);
    five_kind.append(&mut full_house);
    five_kind.append(&mut three_kind);
    five_kind.append(&mut two_pair);
    five_kind.append(&mut one_pair);
    five_kind.append(&mut high_card);

    for i in 0..five_kind.len() {
        result += five_kind[five_kind.len() - 1 - i].bid * (i as u64 + 1);
    }
    result
}

impl Ord for CardsBid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut last = 0;
        for i in 0..5 {
            last = i;
            if self.cards[i] != other.cards[i] {
                break;
            }
        }
        let other_card = other.cards[last];
        let card = self.cards[last];

        if card > other_card {
            Ordering::Less
        } else if card < other_card {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for CardsBid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut last = 0;
        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                last = i;
                break;
            }
        }
        let other_card = other.cards[last];
        let card = self.cards[last];
        if card > other_card {
            Some(Ordering::Less)
        } else if card < other_card {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}
impl PartialEq for CardsBid {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                return false;
            }
        }
        return true;
    }
}
impl Eq for CardsBid {}

impl Ord for CardsBid2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut last = 0;
        for i in 0..5 {
            last = i;
            if self.cards[i] != other.cards[i] {
                break;
            }
        }
        let other_card = other.cards[last];
        let card = self.cards[last];
        if card == 11 {
            return Ordering::Greater;
        }
        if card > other_card {
            Ordering::Less
        } else if card < other_card {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for CardsBid2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut last = 0;
        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                last = i;
                break;
            }
        }
        let other_card = other.cards[last];
        let card = self.cards[last];
        if card == 11 {
            return Some(Ordering::Greater);
        }
        if other_card == 11 {
            return Some(Ordering::Less);
        }
        if card > other_card {
            Some(Ordering::Less)
        } else if card < other_card {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}
impl PartialEq for CardsBid2 {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                return false;
            }
        }
        return true;
    }
}
impl Eq for CardsBid2 {}

#[test]
fn test_sort() {
    use crate::*;
    let mut set1 = vec![
        CardsBid {
            cards: [10, 5, 5, 11, 5].to_vec(),
            bid: 684,
        },
        CardsBid {
            cards: [12, 12, 12, 11, 14].to_vec(),
            bid: 483,
        },
    ];
    let mut set2 = vec![
        CardsBid {
            cards: [13, 10, 11, 11, 10].to_vec(),
            bid: 220,
        },
        CardsBid {
            cards: [13, 13, 6, 7, 7].to_vec(),
            bid: 28,
        },
    ];
    set1.sort();
    set2.sort();
    let num = set2[0].cards[0];
    println!("2 {num:?}");
    println!("1 {set1:?}");
    println!("2 {set2:?}");
}
