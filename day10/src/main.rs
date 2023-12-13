use std::collections::HashSet;
use std::fs;

fn parse(path: &str) -> SearchPath {
    let path: SearchPath = SearchPath::from_grid(
        fs::read_to_string(path)
            .expect("Should have been able to read_to_string(path)")
            .lines()
            .map(|l| split_line(l))
            .collect(),
    );
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
    grid.push(vec!['.'; grid[0].len()]);
    for row in &mut grid {
        row.insert(0, '.');
        row.push('.');
    }
    grid
}

fn get_surrounds(grid: &Vec<Vec<char>>, current: (usize, usize)) -> Vec<Vec<char>> {
    let (ridx, cidx) = current;
    let mut surrounds = vec![];
    for row in grid[ridx - 1..ridx + 2].into_iter() {
        // println!("{:?}", row);
        surrounds.push(row[cidx - 1..cidx + 2].to_vec());
    }
    surrounds
}

struct SearchPath {
    current_char: char,
    previous_char: char,
    current: (usize, usize),
    previous: (usize, usize),
    path: Vec<char>,
    grid: Vec<Vec<char>>,
    nodes: Vec<(usize, usize)>,
}

impl SearchPath {
    fn from_grid(grid: Vec<Vec<char>>) -> SearchPath {
        let grid = pad_grid(&grid);
        let current = find_start(&grid);
        let previous = current;
        let path = vec![];
        let nodes = vec![];
        let current_char = 'S';
        let previous_char = 'S';
        SearchPath {
            current_char,
            previous_char,
            current,
            path,
            grid,
            previous,
            nodes,
        }
    }

    fn next(&mut self) {
        let surrounds = get_surrounds(&self.grid, self.current);
        let valid_north = vec!['S', '|', 'L', 'J'];
        let valid_south = vec!['S', '|', 'F', '7'];
        let valid_west = vec!['S', '-', 'F', 'L'];
        let valid_east = vec!['S', '-', 'J', '7'];

        for (ridx, row) in surrounds.into_iter().enumerate() {
            let rindex = ridx + self.current.0 - 1;
            for (cidx, c) in row.into_iter().enumerate() {
                let cindex = cidx + self.current.1 - 1;

                if (ridx == 0) && (cidx == 1) {
                    if ((c == 'F') | (c == '|') | (c == '7') | (c == 'S'))
                        && valid_north.contains(&self.current_char)
                        && (self.previous != (rindex, cindex))
                    {
                        self.previous = self.current;
                        self.previous_char = self.current_char;
                        self.current = (rindex, cindex);
                        self.path.push(c);
                        self.nodes.push(self.current);
                        self.current_char = c;
                        return;
                    }
                } else if (ridx == 1) && (cidx == 0) {
                    if ((c == 'F') | (c == '-') | (c == 'L') | (c == 'S'))
                        && valid_east.contains(&self.current_char)
                        && (self.previous != (rindex, cindex))
                    {
                        self.previous = self.current;
                        self.previous_char = self.current_char;
                        self.current = (rindex, cindex);
                        self.path.push(c);
                        self.nodes.push(self.current);
                        self.current_char = c;
                        return;
                    }
                } else if (ridx == 1) && (cidx == 2) {
                    if ((c == '7') | (c == '-') | (c == 'J') | (c == 'S'))
                        && valid_west.contains(&self.current_char)
                        && (self.previous != (rindex, cindex))
                    {
                        self.previous = self.current;
                        self.previous_char = self.current_char;
                        self.current = (rindex, cindex);
                        self.path.push(c);
                        self.nodes.push(self.current);
                        self.current_char = c;
                        return;
                    }
                } else if (ridx == 2) && (cidx == 1) {
                    if ((c == 'J') | (c == '|') | (c == 'L') | (c == 'S'))
                        && valid_south.contains(&self.current_char)
                        && (self.previous != (rindex, cindex))
                    {
                        self.previous = self.current;
                        self.previous_char = self.current_char;
                        self.current = (rindex, cindex);
                        self.path.push(c);
                        self.nodes.push(self.current);
                        self.current_char = c;
                        return;
                    }
                }
            }
        }
        print!("Surrounds: {:?}", get_surrounds(&self.grid, self.current));
        panic!("Did not manage to move to the next position")
    }
}

fn move_through_path(mut path: SearchPath) -> SearchPath {
    let mut current_char = ' ';
    while current_char != 'S' {
        path.next();
        current_char = path.current_char;
    }
    println!("Path length: {:?}", path.path.len() / 2);
    path
}

#[derive(PartialEq, Debug)]
enum Direction {
    East,
    North,
    West,
    South,
}

fn get_direction(previous: (usize, usize), current: (usize, usize)) -> Direction {
    if current.1 == previous.1 {
        if current.0 > previous.0 {
            return Direction::South;
        } else {
            return Direction::North;
        }
    } else {
        if current.1 > previous.1 {
            return Direction::East;
        } else {
            return Direction::West;
        }
    }
}

fn is_clockwise(nodes: &Vec<(usize, usize)>) -> bool {
    let mut total = 0;
    for idx in 1..nodes.len() {
        let (y2, x2) = nodes[idx];
        let (y1, x1) = nodes[idx - 1];
        let edge = (x2 as isize - x1 as isize) * (y2 as isize + y1 as isize);
        total += edge;
    }

    total < 0
}

fn pt1() {
    let path = parse("src/input.txt");
    move_through_path(path);
}

fn pt2() {
    let mut path = parse("src/input.txt");
    path = move_through_path(path);

    let mut previous = path.nodes[0];

    let mut search_points: HashSet<(usize, usize)> = HashSet::new();

    let nodes_set: HashSet<(usize, usize)> = HashSet::from_iter(path.nodes.clone());

    let path_is_clockwise = is_clockwise(&path.nodes);
    if !path_is_clockwise {
        path.nodes.reverse();
        path.path.reverse();
    }

    // start by finding the seeds for flood fill. to do this we
    // travel clockwise along the path and find any "seed" (searchable)
    // points along the path
    for (idx, current) in path.nodes.iter().enumerate() {
        if idx == 0 {
            previous = *current;
            continue;
        }
        let direction = get_direction(previous, *current);
        let mut look_at = vec![];

        if direction == Direction::East {
            // look_south
            look_at.push((current.0 + 1, current.1));
            // if i am turning north
            if path.path[idx] == 'J' {
                look_at.push((current.0 + 1, current.1 + 1));
                look_at.push((current.0, current.1 + 1));
            }
            // path.grid
        } else if direction == Direction::West {
            // look_north
            look_at.push((current.0 - 1, current.1));
            // if i am turning south
            if path.path[idx] == 'F' {
                look_at.push((current.0 - 1, current.1 - 1));
                look_at.push((current.0, current.1 - 1));
            }
        } else if direction == Direction::North {
            // look_east
            look_at.push((current.0, current.1 + 1));
            // if i am turning west
            if path.path[idx] == '7' {
                look_at.push((current.0 - 1, current.1 + 1));
                look_at.push((current.0 - 1, current.1));
            }
        } else if direction == Direction::South {
            // look_west
            look_at.push((current.0, current.1 - 1));
            // if i am turning east
            if path.path[idx] == 'L' {
                look_at.push((current.0 + 1, current.1 - 1));
                look_at.push((current.0 + 1, current.1));
            }
        } else {
            panic!("Unknown direction")
        }
        for la in look_at {
            if !nodes_set.contains(&la) {
                search_points.insert(la);
            }
        }

        previous = *current;
    }

    // once we have the seeds we then iteratively extend the seed points
    // into adjacent, unfilled locations until we can extend no longer
    let mut flood_points = HashSet::new();
    println!("Number of seeds: {:?}", search_points.len());
    while flood_points != search_points {
        for p in &flood_points {
            search_points.insert(*p);
        }
        for s in &search_points {
            flood_points.insert(*s);
            let surrounds = get_surrounds(&path.grid, *s);
            for (ridx, row) in surrounds.iter().enumerate() {
                let rindex = ridx + s.0 - 1;
                for (cidx, _c) in row.iter().enumerate() {
                    let cindex = cidx + s.1 - 1;
                    if !nodes_set.contains(&(rindex, cindex)) {
                        flood_points.insert((rindex, cindex));
                    }
                }
            }
        }
        println!("Flooded number of points: {:?}", flood_points.len());
    }
}

fn main() {
    println!("\n=========\nPart 1:\n");
    pt1();
    println!("\n=========\nPart 2:\n");
    pt2();
    println!("\n=========");
}
