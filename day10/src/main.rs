use std::collections::HashMap;
use std::fs;

fn parse(path: &str) -> Vec<Vec<String>> {
    let pipes: Vec<Vec<String>> = fs::read_to_string(path)
        .expect("Should have been able to read_to_string(path)")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>()
        })
        .collect();
    // let clones = pipes.clone();
    pipes
}

fn pt1() {
    let pipes = parse("src/test.txt");
    println!("{:?}", pipes);
}

fn pt2() {}

fn main() {
    println!("\n=========\nPart 1:\n");
    pt1();
    println!("\n=========\nPart 2:\n");
    pt2();
    println!("\n=========");
}
