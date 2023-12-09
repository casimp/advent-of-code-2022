use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
struct Card {
    winners: Vec<i32>,
    selected: Vec<i32>,
    id: usize,
    count: usize,
}

impl Card {
    
    fn matches(&self) -> Vec<i32>{
        let mut correct = vec![];
        for num in &self.selected {
            if self.winners.contains(&num) {
                correct.push(*num as i32)
            }
        }
        correct
    }

    fn spawn(&self, last_id: usize) -> Vec<usize> {
        let num_matches = self.matches().len() as usize;
        let valid_cards: Vec<usize> = (self.id + 1..self.id + num_matches + 1)
            .collect::<Vec<usize>>()
            .into_iter()
            .filter(|c| c <= &last_id)
            .collect();
        valid_cards
    }

    fn add_cards(&mut self, num: &usize) {
        self.count += num;
    }

    fn new(result: &str) -> Self {
        let (card_prefix, game) = result.split_once(": ").unwrap();
        let (_, card_num) = card_prefix.split_once("Card").unwrap();
        let (winners_str, select_str) = game.split_once(" | ").unwrap();
        Self {
            winners: parse_card_numbers(winners_str), 
            selected: parse_card_numbers(select_str),
            id: card_num.trim().parse::<usize>().unwrap(),
            count: 1
        }
    }
}


fn main() {
    let path = Path::new("src/input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => ()
    }

    // break out each line to String
    let lines: Vec<String> = s.lines().map(String::from).collect();
    let mut cards: Vec<Card> = vec![];

    for line in lines {
        let card = Card::new(&line);
        cards.push(card);
    }

    let mut total = 0;
    for idx in 0..cards.len() {
        let card = &cards[idx];
        let count = *&card.count;
        println!("Card {}: {} count and {:?} matches", card.id, count, card.matches().len());
        for id in card.spawn(cards.len()) {
            cards[id -1].add_cards(&count);
        }
        total = total + count;
    }
    println!("Total cards: {:?}", total);
}

fn parse_card_numbers (stringy_numbers: &str) -> Vec<i32> {
    let numbers: Vec<i32> = stringy_numbers
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    numbers
}
