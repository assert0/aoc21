use std::fs;

pub fn part1(line: &str) -> Option<u32> {
    let mut stack = vec![];
    for c in line.chars() {
        match &c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') { return Some(3); }
            },
            ']' => {
                if stack.pop() != Some('[') { return Some(57); }
            },
            '}' => {
                if stack.pop() != Some('{') { return Some(1197); }
            },
            '>' => {
                if stack.pop() != Some('<') { return Some(25137); }
            },
            _ => println!("Unknown char {}", c)
        }
    }
    None
}

pub fn part2(line: &str) -> Option<usize> {
    let mut stack = vec![];
    for c in line.chars() {
        match &c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') { return None; }
            },
            ']' => {
                if stack.pop() != Some('[') { return None; }
            },
            '}' => {
                if stack.pop() != Some('{') { return None; }
            },
            '>' => {
                if stack.pop() != Some('<') { return None; }
            },
            _ => println!("Unknown char {}", c)
        }
    }
    let mut total = 0;
    while let Some(c) = stack.pop() {
        let num = match &c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => { println!("Unknown char {}", c); 0 }
        };
        total = total * 5 + num;
    }
    Some(total)
}

pub fn day10(args: &[String]) -> i32 {
    println!("Day 10");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("Part 1: {}", contents.lines().filter_map(|l| part1(l)).sum::<u32>());
    let mut p = contents.lines().filter_map(|l| part2(l)).collect::<Vec<_>>();
    p.sort();
    println!("Part 2: {:?}", p[p.len()/2]);
    
    0
}
