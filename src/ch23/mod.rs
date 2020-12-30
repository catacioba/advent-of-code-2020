struct Game {
    labels: Vec<u32>,
    current: u32,
    max_value: u32,
}

impl Game {
    fn new(s: &str) -> Game {
        let labels = s
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        Game {
            current: labels[0],
            max_value: *labels.iter().max().unwrap(),
            labels,
        }
    }

    fn remove_next3(&mut self, current_pos: usize) -> Vec<u32> {
        let mut result = Vec::new();

        for _ in 0..3 {
            if current_pos + 1 < self.labels.len() {
                result.push(self.labels.remove(current_pos + 1));
            } else {
                result.push(self.labels.remove(0));
            }
        }

        result
    }

    fn insert(&mut self, values: Vec<u32>) {
        let mut target = self.current - 1;

        target = loop {
            if target == 0 {
                target = self.max_value
            } else if values.contains(&target) {
                target -= 1;
            } else {
                break target;
            }
        };
        println!("destination: {}", target);

        let pos = self.find(target) + 1;
        for idx in 0..3 {
            self.labels.insert(pos + idx, values[idx]);
        }
    }

    fn find(&self, el: u32) -> usize {
        self.labels.iter().position(|x| *x == el).unwrap()
    }

    fn position(&self, p: usize) -> usize {
        p % self.labels.len()
    }

    fn iterate(&mut self) {
        println!("cups: {:?}", self.labels);
        println!("current: {}", self.current);

        let current_pos = self.find(self.current);
        let next = self.remove_next3(current_pos);

        println!("pick up: {:?}", &next);

        self.insert(next);
        let current_pos = self.find(self.current);
        self.current = self.labels[self.position(current_pos + 1)];
    }

    fn value(&self) -> String {
        let pos = self.find(1);
        let mut res = String::new();

        for idx in pos + 1..self.labels.len() {
            res.push_str(&self.labels[idx].to_string());
        }

        for idx in 0..pos {
            res.push_str(&self.labels[idx].to_string());
        }

        res
    }
}

pub fn solve_part1() {
    let mut game = Game::new("685974213");

    for it in 0..100 {
        println!("move {}", it + 1);
        game.iterate();
        println!();
    }

    println!("final: {:?}", game.labels);
    println!("{}", game.value());
}

struct Deck {
    // cards[x] = y => after card x comes card y
    cards: Vec<usize>,
    current: usize,
}

impl Deck {
    fn new(initial_cards: &str, capacity: usize) -> Deck {
        let actual_capacity = capacity + 1;

        let initial_cards_order = initial_cards
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();
        let first_card = initial_cards_order[0];
        let last_card = initial_cards_order[initial_cards_order.len() - 1];

        let mut cards: Vec<usize> = Vec::with_capacity(actual_capacity);

        // grow the vector
        for _ in 0..actual_capacity {
            cards.push(0);
        }

        // modify initial order
        for idx in 0..initial_cards_order.len() - 1 {
            let current = initial_cards_order[idx];
            let next = initial_cards_order[idx + 1];
            cards[current] = next;
        }

        // add the rest of the cards
        let max_value = *initial_cards_order.iter().max().unwrap();
        for el in max_value + 1..actual_capacity - 1 {
            cards[el] = el + 1;
        }

        // link last to the first
        if capacity == initial_cards.len() {
            cards[last_card] = first_card;
        } else {
            cards[last_card] = max_value + 1;
            cards[actual_capacity - 1] = first_card;
        }

        Deck {
            cards,
            current: first_card,
        }
    }

    fn iterate(&mut self) {
        let next_first = self.cards[self.current];
        let next_second = self.cards[next_first];
        let next_third = self.cards[next_second];
        let after = self.cards[next_third];

        let mut target = self.current - 1;
        target = loop {
            if target == 0 {
                target = self.cards.len() - 1;
            } else if target == next_first || target == next_second || target == next_third {
                target -= 1;
            } else {
                break target;
            }
        };
        let target_after = self.cards[target];

        self.cards[self.current] = after;
        self.cards[next_third] = target_after;
        self.cards[target] = next_first;

        self.current = self.cards[self.current];
    }

    fn play(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.iterate();
        }
    }

    fn print(&self) {
        let mut curr = self.current;
        for _ in 0..self.cards.len() - 1 {
            print!("{} ", self.cards[curr]);
            curr = self.cards[curr];
        }
        println!();
    }
}

pub fn solve_part2() {
    let mut deck = Deck::new("685974213", 1000000);
    deck.play(10000000);

    // deck.print();

    println!("{}", deck.cards[1] * deck.cards[deck.cards[1]]);
}

#[cfg(test)]
mod tests {}
