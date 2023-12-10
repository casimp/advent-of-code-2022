use std::fs;

fn parse() -> Vec<Race> {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let (times_str, distances_str) = contents.split_once("\n").expect("Should be able to split");
    let times = times_str[10..]
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());
    let distances = distances_str[10..]
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap());

    let races: Vec<Race> = times
        .zip(distances)
        .map(|(time, record)| Race { time, record })
        .collect();
    races
}

#[derive(Debug)]
struct Race {
    time: i32,
    record: i32,
}

struct RaceStrategy {
    hold: i32,
    time: i32,
}

impl RaceStrategy {
    fn distance(&self) -> i32 {
        self.hold * (self.time - self.hold)
    }
    fn breaks_record(&self, record: i32) -> bool {
        self.distance() > record
    }
}

fn pt1() {
    let races = parse();
    // println!("{:?}", races);
    let mut breaker_product = 1;
    for race in races {
        let mut record_breakers = 0;
        for hold in 0..race.time {
            let strategy = RaceStrategy {
                hold,
                time: race.time,
            };
            if strategy.breaks_record(race.record) {
                record_breakers += 1
            }
        }
        println!("Race breakers: {:?}", record_breakers);
        breaker_product *= record_breakers;
    }
    println!("\nBreaker produce: {:?}", breaker_product);
}

fn pt2() {
    // let contents =
    //     fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
}

fn main() {
    println!("\n=========\nPart 1:\n");
    pt1();
    println!("\n=========\nPart 2:\n");
    pt2();
    println!("\n=========");
}
