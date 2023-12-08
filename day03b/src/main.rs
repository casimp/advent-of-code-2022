use std::fs::File;
use std::io::Read;
use std::path::Path;


struct Number {
    row: usize,
    cols: [usize; 2],
    value: i32,
}

#[derive(Debug)]
pub struct Gear {
    pub row: usize,
    pub col: usize,
    pub numbers: Vec::<i32>,
}

impl Gear {

    fn ratio(&self) -> i32 {
        if self.numbers.len() == 2 {
            self.numbers[0] * self.numbers[1]
        } else { 0 }
    }

    fn add_num_in_range(&mut self, number: &Number) -> () {
        let start_col = number.cols[0] as i32 - 1;
        let end_col = number.cols[1] as i32;
        let start_row = number.row as i32 - 1;
        let end_row = number.row as i32 + 1;

        let gear_col = self.col as i32;
        let gear_row = self.row as i32;

        let valid_col = (gear_col >= start_col) & (gear_col <= end_col);
        let valid_row = (gear_row >= start_row) & (gear_row <= end_row);

        if valid_row && valid_col {
            self.numbers.push(number.value)
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
    let lines: Vec<&str> = s.lines().collect();

    let mut gears = Vec::<Gear>::new();
    for idx in 0..lines.len() {
        let l = *&lines[idx];
        for (cidx, c) in l.chars().enumerate() {
            if c == '*' {
                gears.push(Gear {
                    row: idx, 
                    col: cidx, 
                    numbers: vec![]
            })
            }
        }
    }

    let mut number: Number;
    let mut in_num: bool = false;
    let mut current_number = String::new();
    let mut start_col: usize = 0;

    for idx in 0..lines.len() {
        let l = *&lines[idx];
        // println!("{}", l);
        for (cidx, c) in l.chars().enumerate() {
            let char_is_num = c.is_numeric();
            if char_is_num {
                current_number.push(c);
               if  !in_num  { // entered a number
                in_num = true; 
                start_col = cidx;
            } 
               if cidx == l.len() - 1 { // left a number
                    let value = current_number.parse::<i32>().unwrap();
                    number = Number { row: idx,  cols: [start_col, cidx], value: value };
                    for gear in gears.iter_mut() {
                        gear.add_num_in_range(&number);
                    }
                    in_num = false;
                    current_number.clear();
               }
            } else if !char_is_num & in_num { // left a number
                let value = current_number.parse::<i32>().unwrap();
                number = Number { row: idx,  cols: [start_col, cidx], value: value };
                for gear in gears.iter_mut() {
                    gear.add_num_in_range(&number);
                }
                in_num = false;
                current_number.clear();
            } 
        } 
    }

    let total: i32 = gears.iter().map(| g | g.ratio()).sum();
    println!("{:?}, {:?}", gears[1], gears[2]);
    println!("{:?}", total);
}