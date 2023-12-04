use std::fs::File;
use std::io::Read;
// use std::io::prelude::*;
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
    let numbers: i32 = lines.iter().map(|l| decode_string(&l)).sum();
    println!("Cracked it! The number is {:?}", numbers);

}


fn decode_string(s: &str) -> i32 {
    let s2 = String::from(s);
    let v: Vec<&str> = s2.matches(char::is_numeric).collect();
    let v = format!("{}{}", v[0], v[v.len() - 1]).parse::<i32>().unwrap();
    v
}