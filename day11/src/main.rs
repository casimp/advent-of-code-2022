use std::collections::HashMap;
use std::fs;

fn parse(path: &str) -> Space {
    let space: Space = Space::from_sky_map(
        fs::read_to_string(path)
            .expect("Should have been able to read_to_string(path)")
            .lines()
            .map(|l| split_line(l))
            .collect(),
    );
    space
}

fn split_line(line: &str) -> Vec<char> {
    line.chars().collect()
}

fn get_empty_cols(sky_map: &Vec<Vec<char>>) -> Vec<i64> {
    let num_cols = sky_map.first().unwrap().len();
    let num_rows = sky_map.len();
    let mut empty_cols = vec![];
    for col in 0..num_cols - 1 {
        let mut num_empty = 0;
        for row in sky_map {
            if row[col] != '.' {
                break;
            }
            num_empty += 1;
        }
        if num_empty == num_rows {
            empty_cols.push(col as i64);
        }
    }
    empty_cols
}

fn get_empty_rows(sky_map: &Vec<Vec<char>>) -> Vec<i64> {
    let mut empty_rows = vec![];
    for (idx, row) in sky_map.iter().enumerate() {
        if !row.iter().collect::<String>().contains("#") {
            empty_rows.push(idx as i64);
        }
    }
    empty_rows
}

#[derive(Debug)]
struct Space {
    empty_rows: Vec<i64>,
    empty_cols: Vec<i64>,
    sky_map: Vec<Vec<char>>,
}

impl Space {
    fn from_sky_map(sky_map: Vec<Vec<char>>) -> Space {
        let empty_rows = get_empty_rows(&sky_map);
        let empty_cols = get_empty_cols(&sky_map);
        Self {
            sky_map,
            empty_cols,
            empty_rows,
        }
    }

    fn get_galaxies(&self) -> HashMap<usize, (usize, usize)> {
        let num_cols = self.sky_map.first().unwrap().len();
        let num_rows = self.sky_map.len();
        let mut galaxies = HashMap::new();
        let mut galaxy_counter: usize = 1;
        for ridx in 0..num_rows {
            for cidx in 0..num_cols {
                if self.sky_map[ridx][cidx] == '#' {
                    galaxies.insert(galaxy_counter, (ridx, cidx));
                    galaxy_counter += 1;
                }
            }
        }
        galaxies
    }

    fn get_path_length(&self, start: &(usize, usize), end: &(usize, usize), expand_by: i64) -> i64 {
        let min_row = start.0.min(end.0);
        let max_row = start.0.max(end.0);
        let min_col = start.1.min(end.1);
        let max_col = start.1.max(end.1);

        let row_len = (start.0 as i64 - end.0 as i64).abs();
        let col_len = (start.1 as i64 - end.1 as i64).abs();

        let mut expansion: i64 = 0;
        let rpath = min_row..max_row;
        for r in &self.empty_rows {
            if rpath.contains(&(*r as usize)) {
                expansion += 1;
            }
        }
        let cpath = min_col..max_col;
        for c in &self.empty_cols {
            if cpath.contains(&(*c as usize)) {
                expansion += 1;
            }
        }
        row_len + col_len + expansion * expand_by
    }

    fn get_galaxy_spacings(&self, expand_by: i64) -> i64 {
        let galaxies = self.get_galaxies();
        let mut total_count = 0;
        for g in galaxies.values() {
            let count = galaxies
                .values()
                .map(|v| self.get_path_length(v, g, expand_by))
                .reduce(|a, b| a + b)
                .unwrap();
            total_count += count;
        }
        total_count / 2
    }
}

fn pt1() {
    let space = parse("src/input.txt");
    let expand_by: i64 = 1;
    println!(
        "Galaxy spacings: {:?}",
        space.get_galaxy_spacings(expand_by)
    );
}

fn pt2() {
    let space = parse("src/input.txt");
    let expand_by: i64 = 1000000 - 1;
    println!(
        "Galaxy spacings: {:?}",
        space.get_galaxy_spacings(expand_by)
    );
}

fn main() {
    println!("\n=========\nPart 1:\n");
    pt1();
    println!("\n=========\nPart 2:\n");
    pt2();
    println!("\n=========");
}
