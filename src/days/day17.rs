use std::fs;
use regex::Regex;
use itertools::iproduct;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetArea {
    x: (i32, i32),
    y: (i32, i32),
}

impl TargetArea {

    pub fn parse(line: &str) -> TargetArea {    
        lazy_static! {
            static ref RE: Regex = Regex::new(r"target area: x=(\d+)\.\.(\d+), y=(\-?\d+)\.\.(\-?\d+)").unwrap();
        }
        // Parse line (ex: "target area: x=20..30, y=-10..-5")
        let caps = RE.captures(line).unwrap();
        
        let x0 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let x1 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let y0 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let y1 = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
    
        TargetArea { x: (x0, x1), y: (y0, y1) }
    }

    pub fn in_target(&self, x: i32, y: i32) -> bool {
        x >= self.x.0 && 
        x <= self.x.1 && 
        y >= self.y.0 && 
        y <= self.y.1
    }
    
    pub fn missed_target(&self, x:i32, y: i32) -> bool {
        x > self.x.1 || y < self.y.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Probe {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    maxy: i32,
}

impl Probe {

    pub fn new(dx: i32, dy: i32) -> Probe {    
        Probe { x: 0, y: 0, dx: dx, dy: dy, maxy: 0 }
    }

    pub fn next(&mut self) -> (i32, i32) {
        if self.dx > 0 {
            self.x += self.dx;
            self.dx -= 1;
        }
        self.y += self.dy;
        self.dy -= 1;
        if self.y > self.maxy {
            self.maxy = self.y
        }
        (self.x, self.y) 
    }
    
}


pub fn day17(args: &[String]) -> i32 {
    println!("Day 17");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let target = TargetArea::parse(&contents);
    
    //println!("{:?}", target);
    //println!("{}", target.in_target(28, -7));

    let mut maxy = 0;
    let mut valid = 0;
    for (x, y) in iproduct!(1..500, -100..500) {
        let mut probe = Probe::new(x, y);
        loop {
            let pos = probe.next();
            //println!("{:?} -> {}", pos, target.in_target(pos.0, pos.1));

            if target.missed_target(pos.0, pos.1) {
                break;
            }
            if target.in_target(pos.0, pos.1) {
                valid += 1;
                if probe.maxy > maxy {
                    maxy = probe.maxy;
                }
                break;
            }
        }
    }
    println!("Part 1: {}", maxy);
    println!("Part 2: {}", valid);
    0
}
