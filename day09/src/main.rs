use std::collections::HashMap;
use std::fs;

fn parse(path: &str) -> Vec<History> {
    let histories = fs::read_to_string(path)
        .expect("Should have been able to read_to_string(path)")
        .lines()
        .map(|l| History::new(l))
        .collect();
    histories
}

#[derive(Debug)]
struct History {
    values: Vec<isize>,
}

impl History {
    fn new(values: &str) -> Self {
        let mut values: Vec<isize> = values
            .split_whitespace()
            .map(|v| v.parse::<isize>().unwrap())
            .collect();
        values.reverse();

        Self { values }
    }
}

fn gcd(first: isize, second: isize) -> isize {
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

fn multiply_and_divide(r: isize, a: isize, b: isize) -> isize {
    // See http://blog.plover.com/math/choose-2.html for the idea.
    let g = gcd(r.clone(), b.clone());
    r / g.clone() * (a / (b / g))
}

fn binomial(mut n: isize, k: isize) -> isize {
    let mut r: isize = 1;
    let mut d: isize = 1;
    if k > n {
        return 0;
    }
    loop {
        if d > k {
            break;
        }
        r = multiply_and_divide(r, n.clone(), d.clone());
        n = n - 1;
        d = d + 1;
    }
    r
}

fn binomials(n: isize, a: isize, b: isize) -> Vec<isize> {
    // combines the coefficients with the a and b terms
    // https://cp-algorithms.com/combinatorics/binomial-coefficients.html
    let mut bins = vec![];
    // let a: isize = -1;
    for k in 0..n {
        bins.push(binomial(n, k) * a.pow((n - k) as u32) * b.pow((k) as u32))
    }
    bins
}

fn get_binomial_map(n: isize, a: isize, b: isize) -> HashMap<isize, Vec<isize>> {
    let bins = (1..n).into_iter().map(|n| {
        (n, {
            let mut bins: Vec<isize> = binomials(n, a, b);
            bins.push(1);
            bins.reverse();
            bins
        })
    });
    let bin_map: HashMap<isize, Vec<isize>> = HashMap::from_iter(bins);
    bin_map
}

fn get_bottom_level(values: &Vec<isize>, map: &HashMap<isize, Vec<isize>>) -> isize {
    for level in 1..values.len() as isize {
        let bins = map.get(&level).unwrap();
        let next: isize = values.iter().zip(bins.iter()).map(|(a, b)| a * b).sum();
        if next == 0 {
            return level;
        }
    }
    panic!("Couldn't find the level!")
}

fn get_top_from_bottom(vals: &Vec<isize>, map: &HashMap<isize, Vec<isize>>, lvl: isize) -> isize {
    let bins = map.get(&lvl).unwrap();
    let top: isize = vals.iter().zip(bins[1..].iter()).map(|(a, b)| -a * b).sum();
    top
}

fn pt1() {
    let histories = parse("src/input.txt");
    let n: isize = histories.first().unwrap().values.len() as isize;

    let binomial_map: HashMap<isize, Vec<isize>> = get_binomial_map(n, -1, 1);
    println!("{}: {:?}\n", n, binomial_map);

    let mut top_sum = 0;
    for history in histories {
        let bottom_level = get_bottom_level(&history.values, &binomial_map);
        let top: isize = get_top_from_bottom(&history.values, &binomial_map, bottom_level);
        println!("Level: {}, Top: {:?}", bottom_level, top);
        top_sum += top;
    }
    println!("\nTop sum: {:?}", top_sum);
}

fn pt2() {
    let histories = parse("src/input.txt");
    let n: isize = histories.first().unwrap().values.len() as isize;

    let binomial_map: HashMap<isize, Vec<isize>> = get_binomial_map(n, -1, 1);
    println!("{}: {:?}\n", n, binomial_map);

    let mut top_sum = 0;
    for history in histories {
        let bottom_level = get_bottom_level(&history.values, &binomial_map);
        let mut new_values = history.values.clone();
        new_values.reverse();
        let top: isize = get_top_from_bottom(&new_values, &binomial_map, bottom_level);
        println!("Level: {}, Top: {:?}", bottom_level, top);
        top_sum += top;
    }
    println!("\nTop sum: {:?}", top_sum);
}

fn main() {
    println!("\n=========\nPart 1:\n");
    pt1();
    println!("\n=========\nPart 2:\n");
    pt2();
    println!("\n=========");
}
