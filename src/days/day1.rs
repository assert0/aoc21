use std::fs;
//use itertools::Itertools;

pub fn day1(args: &[String]) -> i32 {
    println!("Day 1");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let entries: Vec<i32> = contents.lines().map(|l| l.parse::<i32>().unwrap()).collect();

    println!("Part 1: {:}", find(&entries, 1));  
    println!("Part 2: {:}", find(&entries, 3));
   
    0
}

pub fn find(entries: &Vec<i32>, windowsize: usize) -> i32 {
    let mut increasing = 0;
    let mut previous = i32::MAX;
    for e in entries.windows(windowsize) {
        let current = e.iter().sum::<i32>();
        if current > previous {
            increasing += 1
        }
        previous = current;
    }
    increasing
}
