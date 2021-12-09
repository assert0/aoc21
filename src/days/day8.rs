use std::fs;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SevenSegment {
    segments: Vec<char>,
}

impl SevenSegment {
    pub fn new(letters: &str) -> SevenSegment {
        SevenSegment { segments: letters.chars().collect() }
    }

    pub fn is_unique(&self) -> bool {
        vec![2, 3, 4, 7].contains(&self.len())
    }

    pub fn len(&self) -> usize {
        self.segments.len()
    }

    pub fn get_number(segment: SevenSegment) -> Option<usize> {
        let mut l = segment.segments.clone();
        l.sort();
        match l.iter().collect::<String>().as_str() {
            "abcefg" => Some(0),
            "cf" => Some(1),
            "acdeg" => Some(2),
            "acdfg" => Some(3),
            "bcdf" => Some(4),
            "abdfg" => Some(5),
            "abdefg" => Some(6),
            "acf" => Some(7),
            "abcdefg" => Some(8),
            "abcdfg" => Some(9),
            _ => None
        }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    patterns: Vec<SevenSegment>,
    outputs: Vec<SevenSegment>,
    mappings: Vec<(char, char)>
}

impl Entry {

    pub fn parse(line: &str) -> Entry {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-g\s]+)\s\|\s([a-g\s]+)").unwrap();
        }
        // Parse line (ex: "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")
        let caps = RE.captures(line).unwrap();
        let patterns: Vec<SevenSegment> = caps.get(1).unwrap().as_str().split_whitespace().map(|t| SevenSegment::new(t)).collect();
        assert_eq!(patterns.len(), 10);
        let outputs: Vec<SevenSegment> = caps.get(2).unwrap().as_str().split_whitespace().map(|t| SevenSegment::new(t)).collect();
        assert_eq!(outputs.len(), 4);
        
        let mappings = Entry::create_mapping(&patterns);
        Entry { patterns, outputs, mappings: mappings }
    }

    pub fn counts(patterns: &Vec<SevenSegment>, c: char) -> usize {
        patterns.iter().map(|p| p.segments.clone()).flatten().filter(|&s| s==c).count()
    }

    pub fn create_mapping(patterns: &Vec<SevenSegment>) -> Vec<(char, char)> {
        let mut mappings = vec![];
        assert_eq!(patterns.len(), 10, "There must be 10 patterns");
        // first map the unique segments by their occurrence counts
        let mut unused = vec![];
        for c in vec!['a','b','c','d','e','f','g'] {
            match Entry::counts(&patterns, c) {
                4 => mappings.push(('e', c)),
                6 => mappings.push(('b', c)),
                9 => mappings.push(('f', c)),
                _ => unused.push(c),
            }
        }
        // then segment 'c' is the only unused letter in the pattern where the count is 2
        let mut unused2 = vec![];
        let p = patterns.iter().find(|p| p.len()==2).unwrap();
        for u in &unused {
            if p.segments.contains(u) {
                mappings.push(('c', *u));
            } else {
                unused2.push(*u);
            }
        }
    
        // then segment 'a' is the only unused letter in the pattern where the count is 3
        let mut unused3 = vec![];
        let p = patterns.iter().find(|p| p.len()==3).unwrap();
        for u in &unused2 {
            if p.segments.contains(u) {
                mappings.push(('a', *u));
            } else {
                unused3.push(*u);
            }
        }
            
        // then segment 'd' is the only unused letter in the pattern where the count is 4
        let mut unused4 = vec![];
        let p = patterns.iter().find(|p| p.len()==4).unwrap();
        for u in &unused3 {
            if p.segments.contains(u) {
                mappings.push(('d', *u));
            } else {
                unused4.push(*u);
            }
        }
        // that only leaves segment 'g'
        mappings.push(('g', unused4[0]));
        return mappings;
    }

    pub fn unmap(&self, c: char) -> char {
        self.mappings.iter().find(|(_, l)| l==&c).unwrap().0
    }

    pub fn fix(&self, segment: &SevenSegment) -> SevenSegment {
        let segments = segment.segments.iter().map(|s| self.unmap(*s)).collect();
        SevenSegment { segments }
    }

    pub fn number(&self) -> usize {
        concat(&self.outputs.iter().map(|o| SevenSegment::get_number(self.fix(o)).unwrap()).collect())
    }

}

// convert vec![1, 2, 3, 4] to 1234
fn concat(numbers: &Vec<usize>) -> usize {
    numbers.iter().fold(0, |acc, n| acc * 10 + n)
}

pub fn day8(args: &[String]) -> i32 {
    println!("Day 8");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let entries: Vec<_> = contents.lines().map(|l| Entry::parse(l)).collect();

    println!("Part 1: {}", entries.iter()
        .map(|e| e.outputs.clone()).flatten().filter(|o| o.is_unique()).count());

    println!("Part 2: {}", entries.iter()
        .map(|e| e.number()).sum::<usize>());
    
    0
}
