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
    let lines: Vec<&str> = s.lines().collect();

    let mut in_num: bool = false;
    let mut current_number = String::new();
    let mut valid_numbers = Vec::<i32>::new();
    let mut num_is_valid: bool = false;

    for idx in 0..lines.len() {
        let l = *&lines[idx];
        println!("{}", l);
        for (cidx, c) in l.chars().enumerate() {
            let char_is_num = c.is_numeric();
            if char_is_num {
                current_number.push(c);
                num_is_valid = check_valid(&lines, idx, cidx) | num_is_valid;
               if  !in_num  { // entered a number
                in_num = true;
               }
               if cidx == l.len() - 1 {
                    println!("{}, {}", current_number, num_is_valid);
                    if num_is_valid {
                        valid_numbers.push(current_number.parse::<i32>().unwrap());
                    }
                    current_number.clear();
                    in_num = false;
                    num_is_valid = false;
                    println!("==========");
               }
            } else if !char_is_num & in_num   { // left a number
                println!("{}, {}", current_number, num_is_valid);
                if num_is_valid {
                    valid_numbers.push(current_number.parse::<i32>().unwrap());
                }
                in_num = false;
                current_number.clear();
                num_is_valid = false;
                println!("==========");
            } 
        } 
    }
    let total: i32 = valid_numbers.iter().sum();
    println!("Valid numbers total: {:?}", total);

}


fn is_symbol(c: char) -> bool {
    !c.is_numeric() & (c != '.')
}

fn check_valid(lines: &Vec<&str>, row: usize, col: usize) -> bool {
    let min_row: usize = if row == 0 { 0 } else { row - 1 };
    let max_row: usize = if row == lines.len() - 1  { lines.len() - 1 } else { row + 1 };
    let min_col: usize = if col == 0 { 0 } else { col - 1 };
    let max_col: usize = if col == lines[0].len() - 1  { lines[0].len() - 1 } else { col + 1 };
    
    let mut surrounds = String::new();
    for l in &lines[min_row..max_row + 1] {
        surrounds.push_str(&l[min_col..max_col + 1]);
    } 
    let symbols: Vec<_> = surrounds.chars().filter(|c| is_symbol(*c)).collect();
    let is_valid = symbols.len() > 0;
    println!("{:?}, {}", surrounds, is_valid);
    is_valid
}
