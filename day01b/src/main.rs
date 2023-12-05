use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;




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
    let num_strs = vec![ 
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "1", "2", "3", "4", "5", "6", "7", "8", "9" ];
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let zipped: Vec<(&&str, &i32)> = num_strs.iter().zip(nums.iter()).collect();
    let str_map: HashMap<_, _> = zipped.into_iter().collect();

    let mut first_idx: i32 = 1000;
    let mut first: &str = "";

    for i in num_strs.clone().iter() {
        match s.find(i) {
            Some(n) => {
                let is_first = first_idx > n as i32;
                first_idx = if is_first { n as i32 } else { first_idx };
                first = if is_first { i } else { first };
        },
            None => ()
        }
    };

    let mut end_idx: i32 = -1;
    let mut end: &str = "";
    for i in num_strs.iter() {
        match s.rfind(i) {
            Some(n) => {
                let is_last = n as i32 > end_idx;
                end_idx = if is_last { n as i32 } else { end_idx };
                end = if is_last { i } else { end };
        },
            None => ()
        }
    };
    let v = format!("{}{}", str_map[&first], str_map[&end]).parse::<i32>().unwrap();
    v
}
