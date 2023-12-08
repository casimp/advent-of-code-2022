use std::fs::File;
use std::io::Read;
use std::path::Path;


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

    // for each line (game) get the winners and selected values
    // then check for the intersection of those values
    for line in lines {
        let mut correct = vec![];
        let (_, game) = line.split_once(": ").unwrap();
        let (winners_str, select_str) = game.split_once(" | ").unwrap();
        let winners: Vec<i32> = winners_str
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let selected: Vec<i32> = select_str
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        println!("WINNERS: {:?}  OURS: {:?}", winners, selected);

        for num in selected {
            if winners.contains(&num) {
                correct.push(num)
            }
        }
        println!("Correct guesses: {:?}", correct);
        let num_correct: i32 = correct.len() as i32;
        let points = if num_correct == 0 {0} else {2i32.pow(num_correct.try_into().unwrap()) / 2};
        println!("Points: {:?}", points);
        total = total + points;
    }
    println!("Total: {:?}", total);
}
