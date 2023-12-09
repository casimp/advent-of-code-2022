use std::fs::File;
use std::io::Read;
use std::path::Path;


struct Card {
    winners: Vec<i32>,
    selected: Vec<i32>,
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

    fn points(&self) -> i32{
        if self.matches().len() > 0 {
            2i32.pow(self.matches().len().try_into().unwrap()) / 2
        } else { 0 }
    }

    fn new(result: &str) -> Self {
        let (_, game) = result.split_once(": ").unwrap();
        let (winners_str, select_str) = game.split_once(" | ").unwrap();
        Self {
            winners: parse_card_numbers(winners_str), 
            selected: parse_card_numbers(select_str),
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
    let mut total = 0;

    for line in lines {
        let card = Card::new(&line);
        println!("Points: {:?}", card.points());
        total = total + card.points();
    }
    println!("Total: {:?}", total);
}

fn parse_card_numbers (stringy_numbers: &str) -> Vec<i32> {
    let numbers: Vec<i32> = stringy_numbers
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    numbers
}