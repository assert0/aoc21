use std::fs;
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

pub fn find_paths_part1(connections: &HashMap<Cave, Vec<Cave>>, path: Vec<Cave>) -> Vec<Vec<Cave>> {
    if *path.last().unwrap() == Cave::end() {
        return vec![path.clone()];
    }
    let next_conn = connections.get(path.last().unwrap()).unwrap();
    let mut paths = vec![];
    for n in next_conn {
        if !path.contains(n) || n.is_big() {
            let mut new_path = path.clone();
            new_path.push(n.clone());
            paths.append(&mut find_paths_part1(&connections, new_path));
        } 
    }
    paths
}

pub fn find_paths_part2(connections: &HashMap<Cave, Vec<Cave>>, small_counts: HashMap<Cave, u32>, path: Vec<Cave>) -> Vec<Vec<Cave>> {
    if *path.last().unwrap() == Cave::end() {
        return vec![path.clone()];
    }
    let next_conn = connections.get(path.last().unwrap()).unwrap();
    let mut paths = vec![];
    for n in next_conn {
        let mut add = true;
        let mut new_small_counts = small_counts.clone();
        if !n.is_big() {
            *new_small_counts.entry(n.clone()).or_default() += 1;
            add = new_small_counts.iter().all(|(_, c)| *c==1) || 
                new_small_counts.iter().filter(|&(_, c)| *c == 2).count() == 1;
        }
        if add {
            let mut new_path = path.clone();
            new_path.push(n.clone());
            paths.append(&mut find_paths_part2(&connections, new_small_counts, new_path));
        }
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

    let mut connections: HashMap<Cave, Vec<Cave>> = HashMap::new();
    contents.lines().for_each(|l| {
        let caves: Vec<Cave> = l.split("-").map(|c| Cave::new(c)).collect();
        if !caves[0].is_end() && !caves[1].is_start() {
            connections.entry(caves[0].clone()).or_default().push(caves[1].clone());
        }
        if !caves[0].is_start() && !caves[1].is_end() {
            connections.entry(caves[1].clone()).or_default().push(caves[0].clone());
        }
    });
   
    println!("Part 1: {}", find_paths_part1(&connections, vec![Cave::start()]).len());
    println!("Part 2: {}", find_paths_part2(&connections, HashMap::new(), vec![Cave::start()]).len());

    0
}
