use std::fs;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    grid: Vec<Vec<char>>,
    infinite_fill: char,
}

impl Image {

    pub fn new(grid: Vec<Vec<char>>) -> Image {        
        Image { grid: grid, infinite_fill: '.' }
    }

    pub fn parse(input :&str) -> Image {
        Image::new(input.lines()
            .map(|l| l.chars().collect()).collect())
    }
    
    pub fn size(&self) -> (usize, usize) {
        (self.grid[0].len(), self.grid.len())
    }

    pub fn get_pixel(&self, x: isize, y: isize) -> char {
        if x >= 0 && x < self.grid[0].len() as isize && 
                y >=0 && y < self.grid.len() as isize {
            return self.grid[y as usize][x as usize];
        }
        self.infinite_fill
    }

    pub fn adjacent(&self, x: isize, y: isize) -> Vec<char> {
        (-1..=1).map(|dy| 
            (-1..=1).map(|dx| 
                self.get_pixel(x + dx, y + dy)).collect::<Vec<char>>()
            ).flatten().collect()
    }

    pub fn lit(&self) -> usize {
        self.grid.iter().flatten().filter(|&p| *p == '#').count()
    }

}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = Vec::new();
        let (xsize, ysize) = self.size();
        for y in 0..ysize {
            for x in 0..xsize {
                output.push(format!("{}", self.grid[y][x]));
            }
            output.push(String::from("\n"));
        }
        write!(f, "{}", output.join(""))
    }
}

// convert ...#...#. to 34
pub fn convert(pixels: &Vec<char>) -> usize {
    pixels.iter().fold(0, |acc, n| (acc << 1) | (*n == '#') as usize)
}

pub fn enhance(algorithm: &Vec<char>, image: &mut Image, round: u32) -> Image {
    let (xsize, ysize) = image.size();
    if *algorithm.first().unwrap() == '#' && *algorithm.last().unwrap() == '.' {
        image.infinite_fill = match round % 2 {
            1 => '#',
            _ => '.',
        };
    }
    let result: Vec<Vec<char>> = (0..ysize+2).map(|y| 
        (0..xsize+2).map(|x|
            algorithm[convert(&image.adjacent(x as isize - 1, y as isize - 1))]).collect::<Vec<char>>()
        ).collect();
    Image::new(result)
}

pub fn day20(args: &[String]) -> i32 {
    println!("Day 20");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut groups = contents.split("\n\n");
    let algorithm: Vec<_> = groups.next().unwrap().chars().collect();
    assert_eq!(algorithm.len(), 512);
    let image = Image::parse(groups.next().unwrap());
    
    let mut part1 = image.clone();
    for i in 0..2 {
        part1 = enhance(&algorithm, &mut part1, i);
        //println!("{}", image);
    }
    println!("Part 1: {}", part1.lit());

    let mut part2 = image.clone();
    for i in 0..50 {
        part2 = enhance(&algorithm, &mut part2, i);
    }
    println!("Part 2: {}", part2.lit());
    
    0
}
    