use std::fs;
use regex::Regex;
use itertools::iproduct;
use std::cmp;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

impl Region {

    pub fn union(&self, other: &Region) -> Region {
        let xmin = cmp::max(self.x.0, other.x.0);
        let xmax = cmp::min(self.x.1, other.x.1);
        let ymin = cmp::max(self.y.0, other.y.0);
        let ymax = cmp::min(self.y.1, other.y.1);
        let zmin = cmp::max(self.z.0, other.z.0);
        let zmax = cmp::min(self.z.1, other.z.1);
        Region { x: (xmin, xmax), y: (ymin, ymax), z: (zmin, zmax) }   
    }

    pub fn is_overlapping(&self, other: &Region) -> bool {
        if other.x.0 >= self.x.0 && other.x.0 <= self.x.1 {
            return true;
        }
        if other.x.1 >= self.x.0 && other.x.1 <= self.x.1 {
            return true;
        }
        if other.y.0 >= self.y.0 && other.y.0 <= self.y.1 {
            return true;
        }
        if other.y.1 >= self.y.0 && other.y.1 <= self.y.1 {
            return true;
        }
        if other.z.0 >= self.z.0 && other.z.0 <= self.z.1 {
            return true;
        }
        if other.z.1 >= self.z.0 && other.z.1 <= self.z.1 {
            return true;
        }  
        false   
    }

    pub fn cubes(&self) -> usize {
        let (x, y, z) = self.size();
        x * y * z
    }

    pub fn size(&self) -> (usize, usize, usize) {
        ((self.x.1 - self.x.0 + 1) as usize, (self.y.1 - self.y.0 + 1) as usize, (self.z.1 - self.z.0 + 1) as usize)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RebootStep {
    on: bool,
    region: Region,
}

impl RebootStep {

    pub fn new(on: bool, region: Region) -> RebootStep {        
        RebootStep { on, region }
    }

    pub fn parse(line :&str) -> RebootStep {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(on|off) x=(\-?\d+)\.\.(\-?\d+),y=(\-?\d+)\.\.(\-?\d+),z=(\-?\d+)\.\.(\-?\d+)").unwrap();
        }
        // Parse line (ex "on x=10..12,y=10..12,z=10..12")
        let caps = RE.captures(line).unwrap();
        let on = match caps.get(1).unwrap().as_str() {
            "on" => true,
            "off" => false,
            _ => panic!("Unknown state"),
        };
        let numbers: Vec<isize> = caps.iter().skip(2).take(6)
            .map(|n| n.unwrap().as_str().parse::<isize>().unwrap()).collect();
        let x = (numbers[0], numbers[1]);
        let y = (numbers[2], numbers[3]);
        let z = (numbers[4], numbers[5]);
        let region = Region { x, y, z };
        RebootStep::new(on, region)
    }

}

pub fn part1(steps: &Vec<RebootStep>) -> usize {
    let region = Region { x: (-50, 50), y: (-50, 50), z: (-50, 50) };
    let (xs, ys, zs) = region.size();
    let (xo, yo, zo) = (50, 50, 50);
    let mut on = vec![vec![vec![false; zs]; ys]; xs];
    for s in steps {
        if region.is_overlapping(&s.region) {
            let u = region.union(&s.region);
            for (x, y, z) in iproduct!(
                                u.x.0..=u.x.1,
                                u.y.0..=u.y.1,
                                u.z.0..=u.z.1) {
                on[(x + xo) as usize][(y + yo) as usize][(z + zo) as usize] = s.on;
            }
        }
    }
    on.iter().flatten().flatten().filter(|&o| *o).count()
}

pub fn day22(args: &[String]) -> i32 {
    println!("Day 22");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let steps: Vec<RebootStep> = contents.lines().map(|l| RebootStep::parse(l)).collect();
   
    println!("Part 1: {}", part1(&steps));
    
    0
}
    