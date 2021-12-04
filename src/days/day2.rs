use std::fs;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    movement: String,
    amount: usize,
}

impl Command {

    pub fn new( movement: String, amount: usize) -> Command {
        Command { movement, amount }
    }

    pub fn parse(command: &str) -> Command {
        // Split out the line (ex: "forward 5")
        let mut parts = command.split(" ");
        let mvt = parts.next().unwrap().to_string();
        let amt = parts.next().unwrap().parse::<usize>().unwrap();
        Command::new(mvt, amt)    
    }
    
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.movement, self.amount)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Submarine {
    part: Part,
    horizontal: usize,
    depth: usize,
    aim: usize,
}

impl Submarine {

    pub fn new(part: Part) -> Submarine {
        Submarine { part: part, horizontal: 0, depth: 0, aim: 0 }
    }

    pub fn execute(&mut self, cmd: &Command) {
        match self.part {
            Part::Part1 => {
                match cmd.movement.as_ref() {
                    "forward" => self.horizontal += cmd.amount,
                    "down" => self.depth += cmd.amount,
                    "up" => self.depth -= cmd.amount,
                    _ => unreachable!("Invalid command: {}", cmd)
                }  
            }
            Part::Part2 => {
                match cmd.movement.as_ref() {
                    "forward" => {
                        self.horizontal += cmd.amount;
                        self.depth += self.aim * cmd.amount;
                    },
                    "down" => self.aim += cmd.amount,
                    "up" => self.aim -= cmd.amount,
                    _ => unreachable!("Invalid command: {}", cmd)
                }  
            }
        }
    }

    pub fn execute_many(&mut self, cmds: &Vec<Command>) -> usize {
        cmds.iter().for_each(|c| self.execute(c));
        self.answer()
    }

    pub fn answer(&self) -> usize {
        self.horizontal * self.depth
    }
    
}

impl fmt::Display for Submarine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{} = {}", self.horizontal, self.depth, self.aim, self.horizontal * self.depth)
    }
}

pub fn day2(args: &[String]) -> i32 {
    println!("Day 2");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let commands: Vec<_> = contents.lines().map(|l| Command::parse(&l)).collect();
    
    let mut sub = Submarine::new(Part::Part1);
    println!("Part 1: {}", sub.execute_many(&commands));

    let mut sub2 = Submarine::new(Part::Part2);
    println!("Part 2: {}", sub2.execute_many(&commands));

    0
}
