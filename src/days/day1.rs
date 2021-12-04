use std::fs;

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

    let entries: Vec<i32> = contents.lines().map(|l| l.parse().unwrap()).collect(); 

    println!("Part 1: {:}", find(&entries, 1));  
    println!("Part 2: {:}", find(&entries, 3));
   
    0
}

pub fn find(entries: &Vec<i32>, windowsize: usize) -> usize {
    let sums: Vec<i32> = entries.windows(windowsize).map(|v| v.iter().sum()).collect();
    sums.iter().zip(sums[1..].iter()).filter(|(a, b)| b > a).count()
}
