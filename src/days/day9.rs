use std::fs;
use itertools::iproduct;

lazy_static! {
    static ref ADJ: Vec<(isize, isize)> = vec![
        (0, 1), (0, -1), (1, 0), (-1, 0)
    ];
}

pub fn adjacent(map: &Vec<Vec<u32>>, y: usize, x: usize) -> Vec<u32> {
    let (my, mx) = (map.len() as isize, map[0].len() as isize); 
    ADJ.iter().map(|(dy, dx)| (y as isize + dy, x as isize + dx))
        .filter(|&(y,  x)| y >= 0 && x >= 0 && y < my && x < mx).map(|(y ,x)| map[y as usize][x as usize])
        .collect()
}

pub fn low_points(map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut l = vec![];
    for (y, x) in iproduct!(0..map.len(), 0..map[0].len()) {
        if adjacent(map, y, x).iter().all(|a| a > &map[y][x]) {
            l.push((y, x));
        }
    }
    l
}

pub fn part1(map: &Vec<Vec<u32>>) -> u32 {
    low_points(&map).iter().map(|&(y, x)| map[y][x] + 1).sum()
}

pub fn basin_size(map: &Vec<Vec<u32>>, used: &mut Vec<Vec<bool>>, y: usize, x: usize) -> usize {
    if map[y][x] == 9 || used[y][x] {
        return 0;
    }
    used[y][x] = true;
    let (my, mx) = (map.len() as isize, map[0].len() as isize);
    ADJ.iter().map(|(dy, dx)| (y as isize + dy, x as isize + dx))
        .filter(|&(y,  x)| y >= 0 && x >= 0 && y < my && x < mx)
        .map(|(y, x)| basin_size(map, used, y as usize, x as usize)).sum::<usize>() + 1
    
}

pub fn part2(map: &Vec<Vec<u32>>) -> usize {
    let mut used = vec![vec![false; map[0].len()]; map.len()];
    let mut sizes: Vec<usize> = low_points(&map).iter()
        .map(|&(y, x)| basin_size(map, &mut used, y, x)).collect();
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

    let map: Vec<Vec<u32>> = contents.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();

    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
    
    0
}
