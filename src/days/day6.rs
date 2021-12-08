use std::fs;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lanternfish {
    timer: u32
}

impl Lanternfish {

    pub fn new(timer: u32) -> Lanternfish {        
        Lanternfish { timer }
    }

    pub fn next_day(&mut self) -> Option<Lanternfish> {
        if self.timer == 0 {
            self.timer = 6;
            return Some(Lanternfish::new(8));
        }
        self.timer -= 1;
        None
    }

}

impl fmt::Display for Lanternfish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.timer)
    }
}

pub fn lantern_count(timer: u32, days: u32) -> usize {
    (timer..days).step_by(7).map(|t| lantern_count(9, days - t)).sum::<usize>() + 1
}

pub fn day6(args: &[String]) -> i32 {
    println!("Day 6");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let days1 = 80;

    let mut fish: Vec<_> = contents.split(",").map(|l| Lanternfish::new(l.parse().unwrap())).collect(); 

    for _ in 0..days1 {
        for j in 0..fish.len() {
            let n = fish[j].next_day();
            if n.is_some() {
                fish.push(n.unwrap());
            }
        }
    }
    println!("Part 1: {}", fish.len());

    let timers: Vec<u32> = contents.split(",").map(|l| l.parse().unwrap()).collect();
    println!("Part 1: {}", timers.iter().map(|&t| lantern_count(t, days1)).sum::<usize>());
    
    let days2 = 256;

    let mut timer_counts: Vec<usize> = vec![0; 6];
    for t in 1..6 {
        timer_counts[t] = lantern_count(t as u32, days2);
    }
    println!("Part 2: {}", timers.iter().map(|&t| timer_counts[t as usize]).sum::<usize>());
       
    0
}


