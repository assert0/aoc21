use std::fs;
use std::fmt;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VentLine {
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl VentLine {

    pub fn parse(line: &str) -> VentLine {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+),(\d+)\s\->\s(\d+),(\d+)").unwrap();
        }
        // Parse line (ex: "242,601 -> 242,18")
        let caps = RE.captures(line).unwrap();
        let vals: Vec<usize> = caps.iter().skip(1).take(4)
            .map(|v| v.unwrap().as_str().parse::<usize>().unwrap()).collect();

        VentLine::new((vals[0], vals[1]), (vals[2], vals[3]))
    }

    pub fn new(start: (usize, usize), end: (usize, usize)) -> VentLine {
        VentLine { start, end }
    }

    pub fn get_range(a: usize, b: usize) -> Vec<usize> {
        assert_ne!(a, b);
        match a < b {
            true => (a..b+1).collect(),
            false => (b..a+1).rev().collect()
        }
    }

    pub fn get_points(&self, diagonal: bool) -> Vec<(usize, usize)> {
        if self.start.0 == self.end.0 {
            return VentLine::get_range(self.start.1, self.end.1).iter()
                .map(|&i| (self.start.0, i)).collect();
        } else if self.start.1 == self.end.1 {
            return VentLine::get_range(self.start.0, self.end.0).iter()
                .map(|&i| (i, self.start.1)).collect();
        } else {
            if diagonal {
                let xr = VentLine::get_range(self.start.0, self.end.0);
                let yr = VentLine::get_range(self.start.1, self.end.1);
                return xr.into_iter().zip(yr.into_iter()).collect();
            } else {
                return vec![];
            }
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VentMap {
    counts: Vec<Vec<u32>>,
}

impl VentMap {

    pub fn new(ventlines: &Vec<VentLine>, diagonal: bool) -> VentMap {
        let xsize = ventlines.iter().map(|v| vec![v.start.0, v.end.0]).flatten().max().unwrap();
        let ysize = ventlines.iter().map(|v| vec![v.start.1, v.end.1]).flatten().max().unwrap();

        let mut m = VentMap { counts: vec![vec![0; xsize + 1]; ysize + 1] };

        ventlines.iter().for_each(
            |v| v.get_points(diagonal).iter().for_each(
                |p| m.counts[p.1][p.0] += 1));
        m
    }

    pub fn overlapping_count(&self, min_count: u32) -> usize {
        self.counts.iter().flatten().filter(|&v| *v >= min_count).count()
    }

}

impl fmt::Display for VentMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = Vec::new();
        for r in &self.counts {
            for c in r {
                if *c == 0 {
                    output.push(String::from("."));
                } else {
                    output.push(format!("{}", c));
                }
            }
            output.push(String::from("\n"));
        }
        write!(f, "{}", output.join(""))
    }
}


pub fn day5(args: &[String]) -> i32 {
    println!("Day 5");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let ventlines: Vec<_> = contents.lines().map(|l| VentLine::parse(l)).collect();

    let ventmap = VentMap::new(&ventlines, false);    
    println!("Part 1: {}", ventmap.overlapping_count(2)); 

    let ventmap2 = VentMap::new(&ventlines, true);
    println!("Part 2: {}", ventmap2.overlapping_count(2));  
    
    0
}
