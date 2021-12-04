use std::fs;
use array2d::Array2D;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rating {
    OxygenGenerator,
    CO2Scrubber,
}

pub fn day3(args: &[String]) -> i32 {
    println!("Day 3");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let entries: Vec<Vec<_>> = contents.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    println!("Part 1: {}", part1(&entries));
    println!("Part 2: {}", part2(&entries));
   
    0
}

fn part1(entries: &Vec<Vec<u32>>) -> u32 {
    let e = Array2D::from_rows(&entries);
    let gamma = e.as_columns().into_iter()
        .map(|col| col.iter().sum::<u32>())
        .map(|s| s > (e.num_rows() as u32) - s) 
        .fold(0, |acc, bit| (acc << 1) | (bit as u32));
    let epsilon = (u32::pow(2, e.num_columns() as u32) - 1) ^ gamma;

    gamma * epsilon
}

fn value_counts(entries: &Vec<Vec<u32>>, bit: usize) -> (u32, u32) {
    let ones = entries.iter().map(|e| e[bit]).sum::<u32>();
    (entries.len() as u32 - ones, ones)
}

fn filter_column(entries: &Vec<Vec<u32>>, bit: usize, rating: Rating) -> u32 {
    if entries.len() == 1 {
        return entries[0].iter().fold(0, |acc, bit| (acc << 1) | bit);
    }
    let (zeros, ones) = value_counts(&entries, bit);
    let value = match rating {
        Rating::OxygenGenerator => if ones >= zeros { 1 } else { 0 },
        Rating::CO2Scrubber => if zeros <= ones { 0 } else { 1 },
    };
    let filtered = entries.iter().filter(|e| e[bit] == value).cloned().collect();

    filter_column(&filtered, bit+1, rating)
}

fn part2(entries: &Vec<Vec<u32>>) -> u32 {
    let oxygen = filter_column(&entries, 0, Rating::OxygenGenerator);
    let co2 = filter_column(&entries, 0, Rating::CO2Scrubber);
    
    oxygen * co2
}