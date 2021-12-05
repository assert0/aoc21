use std::fs;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BingoBoard {
    numbers: Vec<Vec<(u32, bool)>>,
}

impl BingoBoard {

    pub fn parse(board: &str) -> BingoBoard {        
        let numbers: Vec<Vec<u32>> = board.lines().map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect()).collect();
        BingoBoard::new(numbers)
    }

    pub fn new(numbers: Vec<Vec<u32>>) -> BingoBoard {
        for n in &numbers {
            assert_eq!(n.len(), 5);
        }
        assert_eq!(numbers.len(), 5);
        let numbers_bool = numbers.iter().map(|l| l.iter().map(|&n| (n, false)).collect()).collect();
        BingoBoard { numbers: numbers_bool}
    }

    pub fn mark(&mut self, number: u32) -> bool {
        let found = self.numbers.iter_mut()
            .flatten()
            .find(|n| n.0==number);
        if found.is_some() {
            found.unwrap().1 = true;
        }
        self.is_winner()
    }

    pub fn is_winner(&self) -> bool {
        // check rows
        self.numbers.iter()
            .map(|r| r.iter().all(|n| n.1))
            .any(|m| m)
        ||
        // check cols
        (0..self.numbers.len())
            .map(|c| self.numbers.iter().map(|r| r[c]).all(|n| n.1))
            .any(|m| m)
    }

    pub fn score(&self, number: u32) -> Option<u32> {
        if !self.is_winner() {
            return None;
        }
        let sum: u32 = self.numbers.iter()
            .flatten()
            .filter(|&n| !n.1)
            .map(|n| n.0)
            .sum();
        Some(sum * number)
    }
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = Vec::new();
        for r in &self.numbers {
            for c in r {
                if c.1 {
                    output.push(String::from("  X"));
                } else {
                    output.push(format!("{:>3}", c.0));
                }
            }
            output.push(String::from("\n"));
        }
        if self.is_winner() {
            output.push(String::from("Winner!\n"));
        }
        write!(f, "{}", output.join(""))
    }
}

pub fn day4(args: &[String]) -> i32 {
    println!("Day 4");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut groups = contents.split("\n\n");
    let drawn: Vec<u32> = groups.next().unwrap().split(",").map(|n| n.parse().unwrap()).collect();
    let mut boards: Vec<BingoBoard> = groups.map(|b| BingoBoard::parse(b)).collect();

    let mut won = vec![false; boards.len()];
    let mut scores = vec![];
    for n in drawn {
        // println!("Number Drawn: {}", n);
        for (b, w) in &mut boards.iter_mut().zip(won.iter_mut()) {
            if *w {
                continue; // board has already won
            }
            if b.mark(n) {
                // println!("{}", b);
                scores.push(b.score(n).unwrap());
                *w = true;
            }
        }
    }

    println!("Part 1: {}", scores.first().unwrap());  
    println!("Part 2: {}", scores.last().unwrap());  
    
    0
}
