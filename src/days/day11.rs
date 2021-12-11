use std::fs;
use std::fmt;
use itertools::iproduct;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnergyMap {
    map: Vec<Vec<u32>>,
    flashes: usize,
}

lazy_static! {
    static ref ADJ: Vec<(isize, isize)> = vec![
        (0, 1), (0, -1), (1, 0), (-1, 0),
        (1, 1), (1, -1), (-1, 1), (-1, -1)
    ];
}

impl EnergyMap {

    pub fn new(map: Vec<Vec<u32>>) -> EnergyMap {        
        EnergyMap { map, flashes: 0 }
    }

    pub fn parse(input :&str) -> EnergyMap {
        EnergyMap::new(input.lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect())
    }

    pub fn size(&self) -> (usize, usize) {
        (self.map.len(), self.map[0].len())
    }

    pub fn next(&mut self) -> bool {
        self.map.iter_mut().flatten().for_each(|v| *v += 1);
        let (my, mx) = self.size();
        let mut count = 0;
        loop {
            // loop until there are no more flashes
            let mut flashed = false;
            for (y, x) in iproduct!(0..my, 0..mx) {
                if self.map[y][x] == 0 {
                    continue;
                }
                if self.map[y][x] >= 10 {
                    self.map[y][x] = 0;
                    count += 1;
                    flashed = true;
                    // flash adjacent
                    self.adjacent(y as isize, x as isize).iter()
                        .for_each(|&(y, x)| {
                            // increment if haven't already flashed
                            if self.map[y as usize][x as usize] > 0 {
                                self.map[y as usize][x as usize] += 1;
                            }
                        });
                }
            }
            if !flashed {
                break
            }
        }
        self.flashes += count;
        // return true if all flashed this round
        count == my * mx
    }

    pub fn adjacent(&self, y: isize, x: isize) -> Vec<(isize, isize)> {
        let (my, mx) = self.size();
        ADJ.iter().map(|(dy, dx)| (y + dy, x + dx))
            .filter(|&(y, x)| y >= 0 && x >= 0 && y < my as isize && x < mx as isize)
            .collect()
    }

}

impl fmt::Display for EnergyMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = Vec::new();
        for r in &self.map {
            for &i in r {
                if i > 9 {
                    output.push(String::from("X"));
                } else {
                    output.push(format!("{}", i));
                }
            }
            output.push(String::from("\n"));
        }
        write!(f, "{}", output.join(""))
    }
}

pub fn day11(args: &[String]) -> i32 {
    println!("Day 11");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut em = EnergyMap::parse(&contents);
    (0..100).for_each(|_| { em.next(); } );
    println!("Part 1: {}", em.flashes);

    em = EnergyMap::parse(&contents);
    let mut steps = (1..).skip_while(|_| !em.next());
    println!("Part 2: {}", steps.next().unwrap());
    
    0
}
