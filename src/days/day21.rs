use std::fs;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    id: usize,
    position: usize,
    score: usize,
}

impl Player {

    pub fn new(id: usize, position: usize) -> Player {        
        Player { id: id, position: position, score: 0 }
    }

    pub fn parse(line :&str) -> Player {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Player (\d+) starting position: (\d+)").unwrap();
        }
        // Parse line (ex "Player 1 starting position: 4")
        let caps = RE.captures(line).unwrap();
        let id = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let position = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        Player::new(id, position)
    }

    pub fn forward(&mut self, value: usize) -> bool {
        let p = (self.position + value) % 10;
        if p == 0 {
            self.position = 10;
        } else {
            self.position = p;
        }
        self.score += self.position;
        self.is_winner()
    }

    pub fn is_winner(&self) -> bool {
        self.score >= 1000
    }

}

pub fn part1(mut players: Vec<Player>) -> usize {
    let mut dice_it = 1..;
    let mut winner = None;
    while winner.is_none() {
        for p in &mut players {
            let sum = dice_it.by_ref().take(3).sum();
            p.forward(sum);
            if p.is_winner() {
                winner = Some(p.id);
                break;
            }
        }
    }
    let rolls = dice_it.next().unwrap() - 1;
    let losing = players.iter().find(|&p| Some(p.id) != winner).unwrap();
    rolls * losing.score
}

pub fn day21(args: &[String]) -> i32 {
    println!("Day 21");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let players: Vec<Player> = contents.lines().map(|l| Player::parse(l)).collect();
   
    println!("Part 1: {}", part1(players.clone()));
    
    0
}
    