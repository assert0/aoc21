use std::fs;
use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Cave {
    pub name: String,
}

impl Cave {

    pub fn new(name: &str) -> Cave {        
        Cave { name: name.to_string() }
    }

    pub fn is_big(&self) -> bool {
        self.name.chars().all(|c| c.is_ascii_uppercase())
    }

    pub fn is_start(&self) -> bool {
        self.name == "start"
    }

    pub fn is_end(&self) -> bool {
        self.name == "end"
    }

    pub fn start() -> Cave {
        Cave::new("start")
    }

    pub fn end() -> Cave {
        Cave::new("end")
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connection {
    start: Cave,
    end: Cave,
}

impl Connection {

    pub fn parse(line: &str) -> Connection {    
        let cave: Vec<Cave> = line.split("-").map(|c| Cave::new(c)).collect(); 
        Connection { start: cave[0].clone(), end: cave[1].clone() }
    }

    pub fn new(start: Cave, end: Cave) -> Connection {
        Connection { start ,end }
    }

}

impl fmt::Display for Connection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.start.name, self.end.name)
    }
}

pub fn find_connection_start(connections: &Vec<Connection>, start: Cave) -> Vec<Connection> {
    connections.iter().filter(|c| c.start == start).map(|c| c.clone()).collect::<Vec<Connection>>()
}

pub fn find_paths_part1(connections: &Vec<Connection>, path: Vec<Cave>) -> Vec<Vec<Cave>> {
    if *path.last().unwrap() == Cave::end() {
        return vec![path.clone()];
    }
    let next_conn = find_connection_start(&connections, path.last().unwrap().clone());
    let mut paths = vec![];
    for n in next_conn {
        if !path.contains(&n.end) || n.end.is_big() {
            let mut new_path = path.clone();
            new_path.push(n.end);
            paths.append(&mut find_paths_part1(&connections, new_path));
        } 
    }
    paths
}

pub fn visited_small_cave_counts(path: &Vec<Cave>) -> HashMap::<Cave, u32> {
    let mut counts: HashMap<Cave, u32> = HashMap::new();
    for c in path {
        if c.is_big() || c.is_start() {
            continue;
        }
        *counts.entry(c.clone()).or_default() += 1;
    }
    counts
}

pub fn is_valid_small_cave_count(path: &Vec<Cave>) -> bool {
    let counts = visited_small_cave_counts(&path);
    // valid is all the small cave counts are 1
    if counts.iter().all(|(_, counts)| *counts == 1) {
        return true;
    }
    // valid if just 1 of the small caves has a count of 2
    if counts.iter().filter(|&(_, count)| *count == 2).count() == 1 {
        return true;
    }
    false
}

pub fn find_paths_part2(connections: &Vec<Connection>, path: Vec<Cave>) -> Vec<Vec<Cave>> {
    if *path.last().unwrap() == Cave::end() {
        return vec![path.clone()];
    }
    if !is_valid_small_cave_count(&path) {
        return vec![];
    }
    let next_conn = find_connection_start(&connections, path.last().unwrap().clone());
    let mut paths = vec![];
    for n in next_conn {
        let mut new_path = path.clone();
        new_path.push(n.end);
        paths.append(&mut find_paths_part2(&connections, new_path));
    }
    paths
}

pub fn day12(args: &[String]) -> i32 {
    println!("Day 12");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut connections: Vec<_> = contents.lines().map(|l| Connection::parse(l)).collect();

    // add connections in the reverse direction
    connections.append(&mut connections.iter()
        .map(|c| Connection::new(c.end.clone(), c.start.clone()))
        .collect::<Vec<_>>()
    );
    // filter out connections that don't have "start" at the start and "end" at the end
    connections = connections.into_iter()
        .filter(|c| !c.end.is_start() && !c.start.is_end())
        .collect::<Vec<_>>();

    println!("Part 1: {}", find_paths_part1(&connections, vec![Cave::start()]).len());
    println!("Part 2: {}", find_paths_part2(&connections, vec![Cave::start()]).len());

    0
}
