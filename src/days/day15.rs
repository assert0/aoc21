use std::fs;
use itertools::iproduct;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

lazy_static! {
    static ref ADJ: Vec<(isize, isize)> = vec![
        (0, 1), (0, -1), (1, 0), (-1, 0)
    ];
}

pub fn adjacent(y: isize, x: isize) -> Vec<(isize, isize)> {
    ADJ.iter().map(|(dy, dx)| (y + dy, x + dx)).collect()
}

pub fn get_value(map: &Vec<Vec<u32>>, y: isize, x: isize) -> u32 {
    let (my, mx) = (map[0].len(), map.len());
    if y >= 0 && x >= 0 && y < my as isize && x < mx as isize {
        return map[y as usize][x as usize];
    }
    u32::MAX
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    risk: u32,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.risk.cmp(&self.risk)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// derived from https://doc.rust-lang.org/std/collections/binary_heap/index.html
pub fn shortest_path(map: &Vec<Vec<u32>>) -> Option<u32> {
    let (ysize, xsize) = (map.len(), map[0].len());
    let mut lowestrisk = vec![vec![u32::MAX; xsize]; ysize];

    let mut heap = BinaryHeap::new();

    // `start` with a zero risk
    lowestrisk[0][0] = 0;
    heap.push(State { risk: 0, position: (0, 0) });

    // Examine lower risk positions first (min-heap)
    while let Some(State { risk, position }) = heap.pop() {

        if position == (ysize - 1, xsize - 1) { 
            return Some(risk); 
        }

        // Important as we may have already found a better way
        if risk > lowestrisk[position.0][position.1] { 
            continue; 
        }

        // For each node we can reach, see if we can find a way with
        // a lower risk going through this node
        for adj in adjacent(position.0 as isize, position.1 as isize) {
            let adj_risk = get_value(&map, adj.0, adj.1);
            if adj_risk == u32::MAX { continue; }
            let next = State { risk: risk + adj_risk, position: (adj.0 as usize, adj.1 as usize) };

            // If so, add it to the frontier and continue
            if next.risk < lowestrisk[next.position.0][next.position.1] {
                heap.push(next);
                // Relaxation, we have now found a better way
                lowestrisk[next.position.0][next.position.1] = next.risk;
            }
        }
    }

    // Goal not reachable
    None
}

pub fn part2_map(map: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let (ysize, xsize) = (map.len(), map[0].len());
    let mut new = vec![vec![0; xsize * 5]; ysize * 5];

    for (by, bx) in iproduct!(0..5 as usize, 0..5 as usize) {
        let add: u32 = (by + bx) as u32;
        for (y, x) in iproduct!(0..ysize, 0..xsize) {
            let mut n = add + map[y][x];
            if n > 9 {
                n = n % 10 + 1;
            }
            new[(by * ysize + y) as usize][(bx * xsize + x) as usize] = n;
        }
    }
    new
}


pub fn day15(args: &[String]) -> i32 {
    println!("Day 15");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let map: Vec<Vec<u32>> = contents.lines()
            .map(|l| l.chars()
                .map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

    println!("Part 1: {}", shortest_path(&map).unwrap());

    let map2 = part2_map(&map);
    println!("Part 2: {}", shortest_path(&map2).unwrap());
    
    0
}
