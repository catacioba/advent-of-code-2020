use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Debug)]
struct Player {
    id: usize,
    deck: VecDeque<usize>,
}

fn parse_player_id(s: &str) -> usize {
    s[7..s.len() - 1].parse().unwrap()
}

impl Player {
    fn from_block(s: &str) -> Player {
        let mut it = s.lines();

        let id = parse_player_id(it.next().unwrap());
        let deck: VecDeque<usize> = it.map(|l| l.parse().unwrap()).collect();

        Player { id, deck }
    }

    fn play<'a>(&'a mut self, other: &'a mut Player) -> &'a Player {
        while !self.deck.is_empty() && !other.deck.is_empty() {
            let my_first = self.deck.pop_front().unwrap();
            let other_first = other.deck.pop_front().unwrap();

            if my_first > other_first {
                self.deck.push_back(my_first);
                self.deck.push_back(other_first);
            } else {
                other.deck.push_back(other_first);
                other.deck.push_back(my_first);
            }
        }

        if self.deck.is_empty() {
            other
        } else {
            self
        }
    }

    fn play_rec<'a>(&'a mut self, other: &'a mut Player) -> &'a Player {
        let mut played_decks: HashSet<(VecDeque<usize>, VecDeque<usize>)> = HashSet::new();

        while !self.deck.is_empty() && !other.deck.is_empty() {
            let game_state = (self.deck.clone(), other.deck.clone());

            if played_decks.contains(&game_state) {
                return self;
            } else {
                played_decks.insert(game_state);
            }

            let my_first = self.deck.pop_front().unwrap();
            let other_first = other.deck.pop_front().unwrap();

            if self.deck.len() >= my_first && other.deck.len() >= other_first {
                let mut self_copy = self.copy_player(my_first);
                let mut other_copy = other.copy_player(other_first);

                let winner = self_copy.play_rec(&mut other_copy);

                if winner.id == self.id {
                    self.deck.push_back(my_first);
                    self.deck.push_back(other_first);
                } else {
                    other.deck.push_back(other_first);
                    other.deck.push_back(my_first);
                }
            } else {
                if my_first > other_first {
                    self.deck.push_back(my_first);
                    self.deck.push_back(other_first);
                } else {
                    other.deck.push_back(other_first);
                    other.deck.push_back(my_first);
                }
            }
        }

        if self.deck.is_empty() {
            other
        } else {
            self
        }
    }

    fn score(&self) -> usize {
        self.deck
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, el)| el * (idx + 1))
            .fold(0, |c, x| c + x)
    }

    fn copy_player(&self, cards: usize) -> Player {
        let deck_copy = self
            .deck
            .iter()
            .take(cards)
            .cloned()
            .collect::<VecDeque<usize>>();
        Player {
            id: self.id,
            deck: deck_copy,
        }
    }
}

fn read_players() -> (Player, Player) {
    let mut players: Vec<Player> = fs::read_to_string("src/ch22/input.txt")
        .unwrap()
        .split("\r\n\r\n")
        .map(|p| Player::from_block(p))
        .collect();

    let player1 = players.swap_remove(0);
    let player2 = players.swap_remove(0);

    (player1, player2)
}

pub fn solve_part1() {
    let (mut player1, mut player2) = read_players();

    let player = player1.play(&mut player2);

    println!(
        "The winner is Player {}. Score: {}",
        player.id,
        player.score()
    );
}

pub fn solve_part2() {
    let (mut player1, mut player2) = read_players();

    let winner = player1.play_rec(&mut player2);

    println!(
        "The winner is Player {}. Score: {}",
        winner.id,
        winner.score()
    );
}
