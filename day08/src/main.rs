use std::collections::{HashMap, VecDeque};
use std::fs;

fn parse(path: &str) -> (Instructions, Vec<Node>) {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read_to_string(path)");

    let (instructions, nodes_str) = contents
        .split_once("\n\n")
        .expect("Should be able to split");

   (Instructions::new(instructions.to_owned()), create_nodes(nodes_str.to_owned()))
}


fn create_nodes(nodes_str: String) -> Vec<Node> {
    let nodes: Vec<Node> = nodes_str
        .lines()
        .map(|l| Node::new(l.to_string()))
        .collect();
    nodes
}

#[derive(Debug)]
struct Node {
    label: String,
    left: String,
    right: String,
}

#[derive(Debug)]
struct NodeCycle {
    label: String,
    offset: usize,
    cycle: usize,
}


#[derive(Debug)]
struct Instructions {
    initial: String,
    instructions: VecDeque<char>,
}

impl Instructions {

    fn new(instructions: String) -> Self {
        Self { instructions: VecDeque::from(instructions.chars().collect::<Vec<char>>()), initial: instructions }
    }

    fn next(&mut self) -> char {
        let next = self.instructions.pop_front().unwrap();
        self.instructions.push_back(next);
        next
    }

    fn reset(&mut self) {
        self.instructions = VecDeque::from(self.initial.chars().collect::<Vec<char>>());
    }
}

impl Node {
    fn new(l: String) -> Self {
        let (label, connections) = l.split_once(" = ").unwrap();
        let (left, right) = connections[1..connections.len() - 1].split_once(", ").unwrap();
        Self { 
            label: label.to_owned(), left: left.to_owned(), right: right.to_owned() }
    }
}

fn pt1() {
    let (mut instructions, nodes) = parse("src/input.txt");
    println!("Instructions: {:?}\n", instructions.instructions);
    let (_, count) = get_count(&mut instructions, &nodes, "AAA", "ZZZ");
    print!("count: {}\n", count);

}

fn get_count(instructions: &mut Instructions, nodes: &Vec<Node>, start_at: &str, finish_at: &str) -> (String, usize) {
    let node_map: HashMap<String, &Node> = nodes.iter().map(|n| (n.label.clone(), n)).collect();
    let mut count = 0;
    let mut node = node_map.get(start_at).unwrap();
    loop {
        count += 1;
        let instruction = instructions.next();
        if instruction == 'L' {
            node = node_map.get(&node.left).unwrap();
        } else {
            node = node_map.get(&node.right).unwrap();
        }
        // println!("{:?}", node);
        if node.label.ends_with(finish_at) {
            break;
        }
    }
    (node.label.clone(), count)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}


fn get_lcm(node_cycles: &Vec<NodeCycle>) -> usize {
    let mut current_lcm = node_cycles.first().unwrap().cycle;
    for n in node_cycles.iter().skip(1) {
        current_lcm = lcm(current_lcm, n.cycle);
    }
    current_lcm

}

fn pt2() {
    let (mut instructions, nodes) = parse("src/input.txt");
    println!("Instructions: {:?}\n", instructions.instructions);
    let start_nodes: Vec<&Node> = nodes.iter().filter(|n| n.label.ends_with("A")).collect();
    let mut node_cycles: Vec<NodeCycle> = vec![];

    for node in start_nodes {
        // did not initially realise that cycles == offset - made this much easier
        let (end_label, offset) = get_count(&mut instructions, &nodes, node.label.as_str(), "Z");
        let (_, cycle) = get_count(&mut instructions, &nodes, end_label.as_str(), end_label.as_str());
        let node_cycle = NodeCycle { label: node.label.clone(), offset, cycle };
        println!("{:?}", node_cycle);
        node_cycles.push(node_cycle);
        instructions.reset();

    }
    let lcm = get_lcm(&node_cycles);
    print!("\nlcm: {}\n", lcm);
}

fn main() {
    println!("\n=========\nPart 1:\n");
    pt1();
    println!("\n=========\nPart 2:\n");
    pt2();
    println!("\n=========");
}
