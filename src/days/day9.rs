use std::fs;
use itertools::iproduct;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeightMap {
    map: Vec<Vec<u32>>
}

lazy_static! {
    static ref ADJ: Vec<(isize, isize)> = vec![
        (0, 1), (0, -1), (1, 0), (-1, 0)
    ];
}

impl HeightMap {

    pub fn new(map: Vec<Vec<u32>>) -> HeightMap {        
        HeightMap { map }
    }

    pub fn parse(input :&str) -> HeightMap {
        HeightMap::new(input.lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect())
    }

    pub fn size(&self) -> (usize, usize) {
        (self.map.len(), self.map[0].len())
    }

    pub fn value(&self, y: isize, x: isize) -> Option<u32> {
        let (my, mx) = self.size();
        if y >= 0 && x >= 0 && y < my as isize && x < mx as isize {
            return Some(self.map[y as usize][x as usize]);
        }
        None
    }

    pub fn adjacent(&self, y: isize, x: isize) -> Vec<u32> {
        ADJ.iter().map(|(dy, dx)| (y + dy, x + dx))
            .filter_map(|(y, x)| self.value(y, x))
            .collect()
    }

    pub fn low_points(&self) -> Vec<(isize, isize)> {
        let mut l = vec![];
        let s = self.size();
        for (y, x) in iproduct!(0..s.0, 0..s.1) {
            if self.adjacent(y as isize, x as isize).iter().all(|a| a > &self.map[y][x]) {
                l.push((y as isize, x as isize));
            }
        }
        l
    }

    pub fn basin_size(&self, used: &mut Vec<Vec<bool>>, y: isize, x: isize) -> usize {
        let (uy, ux) = (y as usize, x as usize);
        if self.map[uy][ux] == 9 || used[uy][ux] {
            return 0;
        }
        used[uy][ux] = true;
        ADJ.iter().map(|(dy, dx)| (y + dy, x + dx))
            .filter(|&(y, x)| self.value(y, x).is_some())
            .map(|(y, x)| self.basin_size(used, y, x)).sum::<usize>() + 1   
    }

}


pub fn part1(hm: &HeightMap) -> u32 {
    hm.low_points().iter().map(|&(y, x)| hm.value(y, x).unwrap() + 1).sum()
}

pub fn part2(hm: &HeightMap) -> usize {
    let (y, x) = hm.size();
    let mut used = vec![vec![false; x]; y];
    let mut sizes: Vec<usize> = hm.low_points().iter()
        .map(|&(y, x)| hm.basin_size(&mut used, y, x)).collect();
    sizes.sort();
    sizes.iter().rev().take(3).product()
}

pub fn day9(args: &[String]) -> i32 {
    println!("Day 9");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let hm = HeightMap::parse(&contents);

    println!("Part 1: {}", part1(&hm));
    println!("Part 2: {}", part2(&hm));
    
    0
}
