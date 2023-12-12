use std::collections::HashMap;
use std::fs;

fn parse(path: &str) -> SearchPath {
    let path: SearchPath = SearchPath::from_grid(
        fs::read_to_string(path)
        .expect("Should have been able to read_to_string(path)")
        .lines()
        .map(|l| split_line(l))
        .collect());
    path
}

fn split_line(line: &str) -> Vec<char> {
    line.chars().collect()
}


fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (ridx, row) in grid.iter().enumerate() {
        for (cidx, c) in row.iter().enumerate() {
            if c == &'S' {
                return (ridx, cidx);
            }
        }
    }
    panic!("Couldn't find the start of the grid")
}


fn pad_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut grid = grid.clone();
    grid.insert(0, vec!['.'; grid[0].len()]);
    grid.push( vec!['.'; grid[0].len()]);
    for row in &mut grid {
        row.insert(0, '.');
        row.push('.');
    }
    grid
}
struct SearchPath {
    current_char: char,
    previous_char: char,
    start: (usize, usize),
    current: (usize, usize),
    previous: (usize, usize),
    path: Vec<char>,
    grid: Vec<Vec<char>>

}

impl SearchPath {
    fn from_grid(grid: Vec<Vec<char>>) -> SearchPath {
        let grid = pad_grid(&grid);
        let start = find_start(&grid);
        let current = start;
        let previous = start;
        let path = vec![];
        let current_char = 'S';
        let previous_char = 'S';
        SearchPath { current_char, previous_char, start, current, path, grid, previous }
    }

    fn get_surrounds(&self) -> Vec<Vec<char>> {
        let (ridx, cidx) = self.current;
        let mut surrounds = vec![];
        for row in self.grid[ridx - 1..ridx + 2].into_iter() {
            // println!("{:?}", row);
            surrounds.push(row[cidx - 1..cidx + 2].to_vec());
        }
        surrounds
    }

    fn next(&mut self) {
        let surrounds = self.get_surrounds();
        let valid_north = vec!['S', '|', 'L', 'J'];
        let valid_south = vec!['S', '|', 'F', '7'];
        let valid_west = vec!['S', '-', 'F', 'L'];
        let valid_east = vec!['S', '-', 'J', '7'];
        
        for (ridx, row) in surrounds.into_iter().enumerate() {
            let rindex = ridx + self.current.0 - 1;
            for (cidx, c) in row.into_iter().enumerate() {
                let cindex = cidx + self.current.1 - 1;
                 
                if (ridx == 0) && (cidx == 1) {
                    if ((c == 'F') | (c == '|') | (c == '7') | (c == 'S')) && valid_north.contains(&self.current_char) && (self.previous != (rindex, cindex))    {
                        println!("Found {:?} at {:?} | rel = {:?}", c, (rindex, cindex), (ridx, cidx));
                        self.previous = self.current;
                        self.previous_char = self.current_char;
                        self.current = (rindex, cindex);
                        self.path.push(c);
                        self.current_char = c;
                        return;
                    }
                } else if (ridx == 1) && (cidx == 0) {
                    if ((c == 'F') | (c == '-') | (c == 'L') | (c == 'S')) && valid_east.contains(&self.current_char) && (self.previous != (rindex, cindex))    {
                        println!("Found {:?} at {:?} | rel = {:?}", c, (rindex, cindex), (ridx, cidx));
                        self.previous = self.current;
                        self.previous_char = self.current_char;
                        self.current = (rindex, cindex);
                        self.path.push(c);
                        self.current_char = c;
                        return;
                    }
                } else if (ridx == 1) && (cidx == 2) {
                    if ((c == '7') | (c == '-') | (c == 'J') | (c == 'S')) && valid_west.contains(&self.current_char) && (self.previous != (rindex, cindex))  {
                        println!("Found {:?} at {:?} | rel = {:?}", c, (rindex, cindex), (ridx, cidx));
                        self.previous = self.current;
                        self.previous_char = self.current_char;
                        self.current = (rindex, cindex);
                        self.path.push(c);
                        self.current_char = c;
                        return;
                    }
                } else if (ridx == 2) && (cidx == 1) {
                    if ((c == 'J') | (c == '|') | (c == 'L') | (c == 'S')) && valid_south.contains(&self.current_char) && (self.previous != (rindex, cindex))   {
                        println!("Found {:?} at {:?} | rel = {:?}", c, (rindex, cindex), (ridx, cidx));
                        self.previous = self.current;
                        self.previous_char = self.current_char;
                        self.current = (rindex, cindex);
                        self.path.push(c);
                        self.current_char = c;
                        return;
                    }
                }
            }
        }
        print!("Surrounds: {:?}", self.get_surrounds());
        panic!("Did not manage to move to the next position")

    }
}

fn pt1() {
    let mut path = parse("src/input.txt");
    for l in &path.grid {
        println!("{:?}", l);
    }
    println!("{:?}", path.current);
    println!("{:?}", path.get_surrounds());
    let mut current_char = ' ';
    while current_char != 'S' {
        path.next();
        current_char = path.current_char;
    }
    println!("Path length: {:?}", path.path.len() / 2);
}

fn pt2() {}

fn main() {
    println!("\n=========\nPart 1:\n");
    pt1();
    println!("\n=========\nPart 2:\n");
    pt2();
    println!("\n=========");
}

