use std::fs;

pub fn delta(a: usize, b: usize) -> usize {
    (a as isize - b as isize).abs() as usize
}

pub fn cost1(crab: &Vec<usize>, position: usize) -> usize {
    crab.iter().map(|&c| delta(c, position)).sum()
}

pub fn cost2(crab: &Vec<usize>, position: usize) -> usize {
    crab.iter()
        .map(|&c| (0..delta(c, position) + 1).sum::<usize>())
        .sum()
}

pub fn day7(args: &[String]) -> i32 {
    println!("Day 7");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let crab: Vec<_> = contents.split(",").map(|l| l.parse().unwrap()).collect(); 

    println!("Part 1: {}", (0..crab.len()).map(|p| cost1(&crab, p)).min().unwrap());
    println!("Part 2: {}", (0..crab.len()).map(|p| cost2(&crab, p)).min().unwrap());

    0
}


