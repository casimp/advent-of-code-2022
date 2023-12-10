use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;

#[derive(Debug)]
struct PairedSeed {
    current_id: usize,
}

impl PairedSeed {
    fn new(id: usize) -> Self {
        Self { current_id: id }
    }

    fn update(&mut self, id: usize) {
        self.current_id = id;
    }
}

#[derive(Debug)]
struct Seed {
    current: String,
    current_id: usize,
    map: HashMap<String, usize>,
}

impl Seed {
    fn new(id: usize) -> Self {
        let current = String::from("seed");
        Self {
            current: current.clone(),
            current_id: id,
            map: HashMap::from([(current, id)]),
        }
    }

    fn update(&mut self, from: &str, to: &str, id: usize) {
        assert!(from == self.current);
        self.map.insert(to.to_string(), id);
        self.current = to.to_string();
        self.current_id = id;
    }
}

#[derive(Debug)]
struct SeedMap {
    from: String,
    to: String,
    ranges: Vec<Vec<usize>>,
}

impl SeedMap {
    fn get_mapped_id(&self, seed: &Seed) -> usize {
        let mut id = seed.current_id;
        for r in &self.ranges {
            if (id >= r[1]) & (id < r[1] + r[2]) {
                id = seed.current_id + r[0] - r[1];
                break;
            }
        }
        id
    }

    fn get_paired_mapped_id(&self, seed: &PairedSeed) -> usize {
        let mut id = seed.current_id;
        for r in &self.ranges {
            if (id >= r[1]) & (id < r[1] + r[2]) {
                id = seed.current_id + r[0] - r[1];
                break;
            }
        }
        id
    }
}

fn create_seeds(seeds_str: &str) -> Vec<Seed> {
    let seeds: Vec<Seed> = unpack_seeds_str(seeds_str)
        .iter()
        .map(|s| Seed::new(*s))
        .collect();
    seeds
}

fn unpack_seeds_str(seeds_str: &str) -> Vec<usize> {
    let unpacked: Vec<_> = seeds_str
        .split_once(": ")
        .expect("Should be able to split seed prefix")
        .1
        .split_whitespace()
        .map(|s| {
            s.parse::<usize>()
                .expect("Should be able to convert str to i32")
        })
        .collect();
    unpacked
}

fn create_maps(maps_str: &str) -> Vec<SeedMap> {
    let mut maps = vec![];
    for map_str in maps_str.split("\n\n") {
        let (name, m) = map_str
            .split_once(":\n")
            .expect("Should be able to split name prefix");
        let ranges: Vec<_> = m.lines().map(|r| get_range(r)).collect();
        let (from, to) = name
            .trim()
            .split_whitespace()
            .next()
            .unwrap()
            .split_once("-to-")
            .expect("Should be able to split name mapping");
        let seed_map = SeedMap {
            ranges,
            from: from.to_string(),
            to: to.to_string(),
        };
        maps.push(seed_map);
    }
    maps
}

fn get_range(range_str: &str) -> Vec<usize> {
    let range: Vec<usize> = range_str
        .split_whitespace()
        .map(|x| {
            x.parse::<usize>()
                .expect("Should be able to parse as usize")
        })
        .collect::<Vec<usize>>();
    range
}

fn pt1() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let (seeds_str, maps_str) = contents.split_once("\n").expect("Should be able to split");
    let mut seeds = create_seeds(seeds_str);
    let seed_maps = create_maps(&maps_str);

    for seed_index in 0..seeds.len() {
        for index in 0..seed_maps.len() {
            let seed_map = &seed_maps[index];
            let id = seed_map.get_mapped_id(&seeds[seed_index]);
            seeds[seed_index].update(seed_map.from.as_str(), seed_map.to.as_str(), id)
        }
    }
    let min_loc = seeds.iter().map(|s| s.current_id);
    println!("\nMinimum location: {:?}", min_loc.min());
}

fn pt2() {
    let contents =
        fs::read_to_string("src/input.txt").expect("Should have been able to read the file");

    let (seeds_str, maps_str) = contents.split_once("\n").expect("Should be able to split");
    let seeds_unpaired = unpack_seeds_str(seeds_str);
    let seed_maps = create_maps(&maps_str);

    let mut min_loc: Option<usize> = None;
    for index in (0..seeds_unpaired.len()).step_by(2) {
        let id = seeds_unpaired[index];
        let num_seeds = seeds_unpaired[index + 1];
        let mut now = SystemTime::now();
        for (idx, s) in (id..id + num_seeds).enumerate() {
            if idx % 10000000 == 0 {
                let elapsed = now.elapsed();
                println! {"Made it {}/{} through SP-{} in {:?}", idx, num_seeds, index, elapsed};
                now = SystemTime::now();
            }
            let mut seed = PairedSeed::new(s);
            for index in 0..seed_maps.len() {
                let seed_map = &seed_maps[index];
                let id = seed_map.get_paired_mapped_id(&seed);
                seed.update(id)
            }
            if min_loc.is_some() {
                min_loc = if min_loc > Some(seed.current_id) {
                    Some(seed.current_id)
                } else {
                    min_loc
                }
            } else {
                min_loc = Some(seed.current_id)
            }
        }
        println!("**Current min location: {:?}**\n", min_loc);
    }
    println!("\nMinimum location: {:?}", min_loc);
}

fn main() {
    println!("\n=========\nPart 1:\n");
    pt1();
    println!("\n=========\nPart 2:\n");
    pt2();
    println!("\n=========");
}
