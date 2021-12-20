use std::fs;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

impl Beacon {

    pub fn parse(line: &str) -> Beacon {
        let p: Vec<i32> = line.split(",")
            .map(|n| n.parse().unwrap())
            .collect();
        Beacon { x: p[0], y: p[1], z: p[2] }
    }

    const ROTATIONS: usize = 24;

    pub fn orientation(x: i32, y: i32, z: i32, r: usize) -> Beacon {
        let n = match r {
            0 => (x,y,z),
            1 => (x,-y,-z),
            2 => (-x,y,-z),
            3 => (-x,-y,z),
            4 => (x,z,-y),
            5 => (x,-z,y),
            6 => (-x,z,y),
            7 => (-x,-z,-y),
            8 => (y,x,-z),
            9 => (y,-x,z),
           10 => (-y,x,z),
           11 => (-y,-x,-z),
           12 => (y,z,x),
           13 => (y,-z,-x),
           14 => (-y,z,-x),
           15 => (-y,-z,x),
           16 => (z,x,y),
           17 => (z,-x,-y),
           18 => (-z,x,-y),
           19 => (-z,-x,y),
           20 => (z,y,-x),
           21 => (z,-y,x),
           22 => (-z,y,x),
           23 => (-z,-y,-x),
            _ => panic!("Invalid rotation")
        };
        Beacon { x: n.0, y: n.1, z: n.2 }
    }

    pub fn delta(a: &Beacon, b: &Beacon) -> Beacon {
        Beacon { x: b.x - a.x, y: b.y - a.y, z: b.z - a.z }
    }

    pub fn manhatten(a: &Beacon, b: &Beacon) -> i32 {
        let d = Beacon::delta(a, b);
        d.x.abs() + d.y.abs() + d.z.abs()
    }
    

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scanner {
    id: usize,
    beacons: Vec<Beacon>,
}

impl Scanner {

    pub fn parse(lines: &str) -> Scanner {
        let mut it = lines.lines();
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\-+ scanner (\d+) \-+").unwrap();
        }
        // Parse line (ex: "--- scanner 17 ---")
        let caps = RE.captures(it.next().unwrap()).unwrap();
        let id: usize = caps.get(1).unwrap().as_str().parse().unwrap();
        
        let beacons = it.map(|l| Beacon::parse(l)).collect();

        Scanner { id, beacons }
    }
   
}

pub fn reorient(beacons: &Vec<Beacon>, o: usize) -> Vec<Beacon> {
    beacons.iter().map(|b| Beacon::orientation(b.x, b.y, b.z, o)).collect()
}

pub fn overlaps(left: &Vec<Beacon>, right: &Vec<Beacon>) -> (Option<Beacon>, Vec<Beacon>) {
    for l in left {
        for o in 0..Beacon::ROTATIONS {
            let reoriented = reorient(&right, o);
            for i in 0..reoriented.len() {
                let d = Beacon::delta(l, &reoriented[i]);
                let shifted = reoriented.iter().map(|a| Beacon::delta(&d, a)).collect::<Vec<Beacon>>();
                let matches = shifted.iter()
                    .filter(|a| left.contains(a)).count();
                if matches >= 12 { 
                    return (Some(d), shifted);
                }
            }
        }
    }
    (None, vec![])
}

pub fn day19(args: &[String]) -> i32 {
    println!("Day 19");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let groups = contents.split("\n\n");
    let scanners: Vec<_> = groups.map(|g| Scanner::parse(g)).collect();
    
    let beacons = &mut scanners[0].beacons.to_vec();
    let mut used: Vec<_> = vec![0];
    let mut deltas: Vec<_> = vec![];
    while used.len() < scanners.len() {
        for u in 0..scanners.len() {
            if used.contains(&u) {
                continue;
            }
            let (delta, foundbeacons) = overlaps(&beacons, &scanners[u].beacons);
            println!("{} {} Delta: {:?} {}", used.len(), u, delta, beacons.len());
            if delta.is_some() {
                used.push(u);
                for f in foundbeacons {
                    if !beacons.contains(&f) {
                        beacons.push(f);
                        deltas.push(delta.unwrap());
                    }
                }
                break;
            }
        }
    }
    println!("Part 1: {}", beacons.len());
    println!("Part 2: {}", deltas.iter().combinations(2).map(|p| Beacon::manhatten(p[0], p[1])).max().unwrap());

    0
}
