use std::fs;
use itertools::Itertools;
use std::collections::HashMap;

type Rules = HashMap<(char, char), char>;

#[derive(Debug)]
struct Polymer {
    pairwise: HashMap<(char, char), usize>,
    counts: HashMap<char, usize>,
}

impl Polymer {
    pub fn new(template: &str) -> Self {
        let mut pairwise: HashMap<(char, char), usize> = HashMap::new();
        for pair in template.chars().tuple_windows::<(char, char)>() {
            *pairwise.entry(pair).or_default() += 1
        }
        let mut counts: HashMap<char, usize> = HashMap::new();
        for c in template.chars() {
            *counts.entry(c).or_default() += 1;
        }
        Self { 
            pairwise,
            counts
        }

    }
    
    pub fn step(&self, rules: &Rules) -> Polymer {
        let mut next_pairwise = HashMap::new();
        let mut next_counts = self.counts.clone();
        self.pairwise.iter().for_each(|(&pair, freq)| {
            if let Some(&e) = rules.get(&pair) {
                *next_counts.entry(e).or_default() += freq;
                *next_pairwise.entry((pair.0, e)).or_default() += freq;
                *next_pairwise.entry((e, pair.1)).or_default() += freq;
            } else {
                panic!("No rule?");
            }
        });
        Self {
            pairwise: next_pairwise,
            counts: next_counts
        }
    }

    fn min(&self) -> usize {
        *self.counts.values().min().unwrap()
    }

    fn max(&self) -> usize {
        *self.counts.values().max().unwrap()
    }

}

pub fn day14(args: &[String]) -> i32 {
    println!("Day 14");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut groups = contents.split("\n\n");
    let template = groups.next().unwrap();
    let mut poly = Polymer::new(&template);
    let mappings: Vec<(&str, &str)> = groups.next().unwrap()
        .lines().map(|l| l.split(" -> ").next_tuple().unwrap()).collect();    
    let mut rules: Rules = HashMap::new();
    for (left, right) in mappings {
        rules.insert(left.chars().next_tuple().unwrap(), right.chars().next().unwrap());
    }

    for _ in 0..10 {
        poly = poly.step(&rules);
    }
    println!("Part 1: {}", poly.max() - poly.min());

    for _ in 10..40 {
        poly = poly.step(&rules);
    }
    println!("Part 2: {}", poly.max() - poly.min());

    0
}
