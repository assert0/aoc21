use std::fs;
use std::fmt;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paper {
    marked: Vec<(usize, usize)>,
    size: (usize, usize),
}

impl Paper {

    pub fn parse(board: &str) -> Paper {
        let marked: Vec<(usize, usize)> = board.lines()
            .map(|l| l.split(",")
                .map(|n| n.parse().unwrap()).next_tuple().unwrap()
            ).collect();
        let xsize = marked.iter().map(|p| p.0).max().unwrap() + 1;
        let ysize = marked.iter().map(|p| p.1).max().unwrap() + 1;
        let size = (xsize, ysize);
        Paper { marked, size }
    }

    pub fn new(marked: Vec<(usize, usize)>, size: (usize, usize)) -> Paper {
        Paper { marked, size }
    }

    pub fn map(&self) -> Vec<Vec<bool>> {
        let (xsize, ysize) = self.size;
        let mut map = vec![vec![false; xsize]; ysize];
        for m in &self.marked {
            map[m.1][m.0] = true
        }
        map
    }
   
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = Vec::new();
        for r in self.map() {
            for c in r {
                if c {
                    output.push(String::from("#"));
                } else {
                    output.push(String::from("."));
                }
            }
            output.push(String::from("\n"));
        }
        write!(f, "{}", output.join(""))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fold {
    axis: String,
    offset: usize,
}

impl Fold {

    pub fn parse(line: &str) -> Fold {    
        lazy_static! {
            static ref RE: Regex = Regex::new(r"fold along (x|y)=(\d+)").unwrap();
        }
        // Parse line (ex: "fold along y=7")
        let caps = RE.captures(line).unwrap();
        let axis = caps.get(1).unwrap().as_str().to_string();
        let offset = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();

        Fold { axis, offset }
    }
    
}

pub fn fold_x(marked: &Vec<(usize, usize)>, fold: usize) -> Vec<(usize, usize)> {
    marked.iter().map(|m| 
        if m.0 < fold {
            (m.0, m.1)
        } else {
            (fold * 2 - m.0, m.1)
        }
    ).collect()
}

pub fn fold_y(marked: &Vec<(usize, usize)>, fold: usize) -> Vec<(usize, usize)> {
    marked.iter().map(|m|
        if m.1 < fold {
            (m.0, m.1)
        } else {
            (m.0, fold * 2 - m.1)
        }
    ).collect()
}

pub fn fold_paper(paper: &Paper, fold: &Fold) -> Paper {
    match fold.axis.as_str() {
        "x" => Paper::new(fold_x(&paper.marked, fold.offset), (fold.offset, paper.size.1)),
        "y" => Paper::new(fold_y(&paper.marked, fold.offset), (paper.size.0, fold.offset)),
        _ => panic!("Invalid axis")
    }
}

pub fn day13(args: &[String]) -> i32 {
    println!("Day 13");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut groups = contents.split("\n\n");
    let mut paper: Paper = Paper::parse(groups.next().unwrap());
    let folds: Vec<Fold> = groups.next().unwrap().lines().map(|l| Fold::parse(l)).collect();
    
    for (i, f) in folds.iter().enumerate() {
        if i == 1 {
            println!("Part 1: {}", paper.map().iter().flatten().filter(|&p| *p).count());
        }
        paper = fold_paper(&paper, f);
    }
    println!("Part 2:");
    println!("{}", paper);

    0
}
