use std::fs;

fn parse_pt1() -> Vec<Race> {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let (times_str, distances_str) = contents.split_once("\n").expect("Should be able to split");
    let times = times_str[10..]
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let distances = distances_str[10..]
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap());

    let races: Vec<Race> = times
        .zip(distances)
        .map(|(time, record)| Race { time, record })
        .collect();
    races
}

fn parse_pt2() -> Race {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");
    let (times_str, distances_str) = contents.split_once("\n").expect("Should be able to split");
    let time: i64 = times_str[10..]
        .split_whitespace()
        .flat_map(|s| s.chars())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let record: i64 = distances_str[10..]
        .split_whitespace()
        .flat_map(|s| s.chars())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    Race { time, record }
}

#[derive(Debug)]
struct Race {
    time: i64,
    record: i64,
}

impl Race {
    fn hold_to_break(&self) -> i64 {
        let a: f64 = -1.0;
        let b: f64 = self.time as f64;
        let c: f64 = -self.record as f64;
        let h: f64 = (-b + (b.powf(2.0) - 4.0 * a * c).powf(0.5)) / (2.0 * a);
        return h.ceil() as i64;
    }
    fn record_breakers(&self) -> i64 {
        let h = self.hold_to_break();
        let record_breakers = self.time + 1 - 2 * h;
        record_breakers
    }
}

fn pt1() {
    let races = parse_pt1();
    // println!("{:?}", races);
    let mut breaker_product = 1;
    for race in races {
        let record_breakers = race.record_breakers();
        println!("Record breakers: {:?}", record_breakers);
        breaker_product *= record_breakers;
    }
    println!("\nBreaker product: {:?}", breaker_product);
}

fn pt2() {
    let race = parse_pt2();
    println!("Race: {:?}\n", race);
    println!("Broken at {:?}", race.hold_to_break());
    println!("Record breakers {:?}", race.record_breakers());
}

fn main() {
    println!("\n=========\nPart 1:\n");
    pt1();
    println!("\n=========\nPart 2:\n");
    pt2();
    println!("\n=========");
}
