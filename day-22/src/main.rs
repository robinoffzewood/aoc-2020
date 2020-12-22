use md5;
use md5::Digest;
use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut tst_play = Combat::from_file("example.txt");
    let mut play = Combat::from_file("input.txt");

    // part 1
    tst_play.run();
    assert_eq!(306, tst_play.score(2));
    let winner = play.run();
    println!("Score (normal) = {}", play.score(winner));

    // part 2
    let mut tst_play = Combat::from_file("example.txt");
    let mut play = Combat::from_file("input.txt");
    let winner = Combat::run_recurse(&mut tst_play.player_1, &mut tst_play.player_2);
    assert_eq!(291, tst_play.score(winner));
    println!(
        "Winner = Player {} with score {}",
        winner,
        tst_play.score(winner)
    );

    let winner = Combat::run_recurse(&mut play.player_1, &mut play.player_2);
    println!(
        "Winner = Player {} with score {}",
        winner,
        play.score(winner)
    );

    let duration = start.elapsed();
    println!("Finished after {:?}", duration);
}

struct Combat {
    player_1: VecDeque<usize>,
    player_2: VecDeque<usize>,
}

impl Combat {
    fn run(&mut self) -> u8 {
        let mut winner = 1;
        loop {
            if self.player_2.len() == 0 || self.player_1.len() == 0 {
                // GAME OVER!
                break;
            }
            let p2_card = self.player_2.pop_front().unwrap();
            let p1_card = self.player_1.pop_front().unwrap();
            if p2_card > p1_card {
                self.player_2.push_back(p2_card);
                self.player_2.push_back(p1_card);
                winner = 2;
            } else {
                self.player_1.push_back(p1_card);
                self.player_1.push_back(p2_card);
                winner = 1;
            }
        }
        winner
    }

    // return 1 if Player 1 win and 2 if Player 2 win
    fn run_recurse(p1_deck: &mut VecDeque<usize>, p2_deck: &mut VecDeque<usize>) -> u8 {
        let mut winner = 1;
        let mut p1_hashtable: Vec<Digest> = Vec::new();
        let mut p2_hashtable: Vec<Digest> = Vec::new();
        loop {
            if p2_deck.len() == 0 || p1_deck.len() == 0 {
                // GAME OVER!
                break;
            }

            // Draw a card from the top of each deck
            let p1_card = p1_deck.pop_front().unwrap();
            let p2_card = p2_deck.pop_front().unwrap();

            // If we haven't enough cards on our deck, compare their value
            if p2_deck.len() < p2_card || p1_deck.len() < p1_card {
                if p2_card > p1_card {
                    winner = 2;
                    p2_deck.push_back(p2_card);
                    p2_deck.push_back(p1_card);
                    // Prevention rule !
                    let deck_hash = Combat::compute_hash(&p2_deck);
                    if p2_hashtable.contains(&deck_hash) {
                        winner = 1;
                        break;
                    }
                    p2_hashtable.push(deck_hash);
                } else {
                    winner = 1;
                    p1_deck.push_back(p1_card);
                    p1_deck.push_back(p2_card);
                    // Prevention rule !
                    let deck_hash = Combat::compute_hash(&p1_deck);
                    if p1_hashtable.contains(&deck_hash) {
                        break;
                    }
                    p1_hashtable.push(deck_hash);
                }
            } else {
                // Enter a new sub game
                // Make copies of the deck, keeping only N first cards
                let mut crab_deck_copy: VecDeque<usize> = p2_deck.clone();
                crab_deck_copy.truncate(p2_card);
                let mut my_deck_copy: VecDeque<usize> = p1_deck.clone();
                my_deck_copy.truncate(p1_card);

                winner = Combat::run_recurse(&mut my_deck_copy, &mut crab_deck_copy);
                if winner == 1 {
                    p1_deck.push_back(p1_card);
                    p1_deck.push_back(p2_card);
                    continue;
                }
                if winner == 2 {
                    p2_deck.push_back(p2_card);
                    p2_deck.push_back(p1_card);
                    continue;
                }
                break;
            }
        }
        winner
    }

    fn compute_hash(deck: &VecDeque<usize>) -> Digest {
        let mut as_str: String = "".to_string();
        for card in deck {
            as_str += &*card.to_string();
        }
        md5::compute(as_str.as_bytes())
    }

    fn score(&self, winner: u8) -> usize {
        let cards_and_weight;
        if winner == 1 {
            cards_and_weight = self.player_1.iter().rev().zip(1..);
        } else {
            cards_and_weight = self.player_2.iter().rev().zip(1..);
        }
        cards_and_weight.map(|(card, i)| card * i).sum()
    }

    fn from_file(f_name: &str) -> Combat {
        // split by empty line, getting each deck
        let str_in = fs::read_to_string(f_name).expect("Error in reading file");
        let decks: Vec<&str> = str_in.split("\r\n\r\n").collect();

        let mut p1_deck = VecDeque::new();
        for card in decks[0].lines() {
            if card.starts_with("Player") {
                continue;
            }
            let card = card.parse::<usize>().expect("invalid card");
            p1_deck.push_back(card);
        }
        let mut p2_deck = VecDeque::new();
        for card in decks[1].lines() {
            if card.starts_with("Player") {
                continue;
            }
            let card = card.parse::<usize>().expect("invalid card");
            p2_deck.push_back(card);
        }
        Combat {
            player_1: p1_deck,
            player_2: p2_deck,
        }
    }
}
