use std::fs;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagram {
    hallway: Vec<char>,
    rooms: Vec<Vec<char>>,
    depth: usize,
}

impl Diagram {

    pub fn new(hallway: Vec<char>, rooms: Vec<Vec<char>>, depth: usize) -> Diagram {        
        Diagram { hallway: hallway, rooms: rooms, depth: depth }
    }

    pub fn parse(input :&str) -> Diagram {
        let m: Vec<Vec<char>> = input.lines()
            .map(|l| l.chars().collect())
            .collect();
        let mut rooms = vec![];
        rooms.push(vec![m[2][3], m[3][3]]);
        rooms.push(vec![m[2][5], m[3][5]]);
        rooms.push(vec![m[2][7], m[3][7]]);
        rooms.push(vec![m[2][9], m[3][9]]);
        Diagram::new(m[1][1..12].to_vec(), rooms, 2)
    }

    pub fn parse2(input :&str) -> Diagram {
        let mut lines = input.lines().collect::<Vec<&str>>();
        lines.insert(3, "  #D#C#B#A#");
        lines.insert(4, "  #D#B#A#C#");
        let m: Vec<Vec<char>> = lines.iter()
            .map(|l| l.chars().collect())
            .collect();
        let mut rooms = vec![];
        rooms.push(vec![m[2][3], m[3][3], m[4][3], m[5][3]]);
        rooms.push(vec![m[2][5], m[3][5], m[4][5], m[5][5]]);
        rooms.push(vec![m[2][7], m[3][7], m[4][7], m[5][7]]);
        rooms.push(vec![m[2][9], m[3][9], m[4][9], m[5][9]]);
        Diagram::new(m[1][1..12].to_vec(), rooms, 4)
    }

    pub fn get_move_from(spots: Vec<char>, correct: char) -> Option<usize> {
        for i in 0..spots.len() {
            if spots[i] == '.' {
                continue;
            }
            if spots[i..].iter().all(|s| *s==correct) {
                return None;
            }
            return Some(i);
        }
        None
    }

    pub fn need_move(&self, room: usize) -> Option<usize> {
        let correct = Diagram::get_room_amphipod(room);
        let spots: Vec<char> = self.rooms[room].clone();

        Diagram::get_move_from(spots, correct)
    }


    pub fn get_room_hallway_position(&self, room: usize) -> usize {
        match room {
            0 => 2,
            1 => 4,
            2 => 6,
            3 => 8,
            _ => panic!()
        }
    }

    pub fn get_room_amphipod(room: usize) -> char {
        match room {
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            _ => panic!()
        }
    }

    pub fn move_into_room(&mut self, from_hallway_postion: usize) -> Option<usize> {
        let amphipod = self.hallway[from_hallway_postion];
        let room = match amphipod {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            _ => panic!("No amphipod")
        };
        if self.rooms[room].iter().all(|&a| a != '.') {
            return None; //no space left
        }
        if self.rooms[room].iter().all(|&a| a == '.' || a == amphipod) {
            let r = self.get_room_hallway_position(room);
            let mut count = 0;
            if from_hallway_postion > r {
                // going left
                for p in (r..from_hallway_postion).rev() {
                    if self.hallway[p] != '.' {
                        return None;
                    }
                    count += 1;
                }
            } else if from_hallway_postion < r {
                // going right
                for p in from_hallway_postion+1..=r {
                    if self.hallway[p] != '.' {
                        return None;
                    }
                    count += 1;
                }
            } else {
                panic!("Invalid");
            }
            // find the correct slot in the room
            let mut i = 0;
            for (p, a) in self.rooms[room].iter().enumerate() {
                if *a == amphipod {
                    break;
                }
                i = p;
                count += 1;
            }
            self.rooms[room][i] = amphipod;
            self.hallway[from_hallway_postion] = '.';
            return Some(count);
        }
        None
    }

    pub fn available_hallway_positions(&self, from_room: usize, room_position: usize) -> Vec<(usize, usize)> {
        let mut available = vec![];
        let s = self.get_room_hallway_position(from_room);
        let mut count = 0;
        // going left
        for p in (0..s).rev() {
            count += 1;
            if vec![2, 4, 6, 8].contains(&p) {
                continue;
            }
            if self.hallway[p] != '.' {
                break;
            }
            available.push((p, count + room_position + 1));
        }
        count = 0;
        // going right
        for p in s+1..11 {
            count += 1;
            if vec![2, 4, 6, 8].contains(&p) {
                continue;
            }
            if self.hallway[p] != '.' {
                break;
            }
            available.push((p, count + room_position + 1));
        }
        available
    }

    pub fn is_done(&self) -> bool {
        self.rooms[0].iter().all(|&c| c=='A') &&
        self.rooms[1].iter().all(|&c| c=='B') &&
        self.rooms[2].iter().all(|&c| c=='C') &&
        self.rooms[3].iter().all(|&c| c=='D')
    }

    pub fn cost_per_move(amphipod: char) -> usize {
        match amphipod {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => panic!("No amphipod")
        }
    }

}

impl fmt::Display for Diagram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = Vec::new();
        output.push(String::from("#############"));
        output.push(format!("#{}#", self.hallway.iter().cloned().collect::<String>()));
        output.push(format!("###{}#{}#{}#{}###", self.rooms[0][0], self.rooms[1][0], self.rooms[2][0], self.rooms[3][0]));
        for i in 1..self.rooms[0].len() {
            output.push(format!("  #{}#{}#{}#{}#", self.rooms[0][i], self.rooms[1][i], self.rooms[2][i], self.rooms[3][i]));
        }
        output.push(String::from("  #########"));
        write!(f, "{}", output.join("\n"))
    }
}

pub fn solution(diagram: Diagram, cost: usize, best: usize) -> usize {
    if diagram.is_done() {
        return cost;
    }
    let mut newbest = best;
    for (p, a) in diagram.hallway.iter().enumerate() {
        if *a != '.' {
            let mut next = diagram.clone();
            let steps = next.move_into_room(p);
            if steps.is_some() {
                let newcost = cost + steps.unwrap() * Diagram::cost_per_move(*a);
                let result = solution(next, newcost, newbest);
                if result < newbest {
                    newbest = result;
                    // println!("Solution: {}", newbest);
                }
            }
        }
    }
    for r in 0..4 {   
        let needs_move = diagram.need_move(r);
        if needs_move.is_some() {
            let slot = needs_move.unwrap();
            let avail = diagram.available_hallway_positions(r, slot);
            for (a, steps) in avail {
                let mut next = diagram.clone();
                let amphipod = diagram.rooms[r][slot];
                next.hallway[a] = amphipod;
                next.rooms[r][slot] = '.';
                let newcost = cost + steps * Diagram::cost_per_move(amphipod);
                let result = solution(next, newcost, newbest);
                if result < best {
                    newbest = result;
                    // println!("Solution: {}", newbest);
                }
            }
        }
    }
    newbest
}

pub fn day23(args: &[String]) -> i32 {
    println!("Day 23");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let d = Diagram::parse(&contents);
    // println!("{}", d);
    println!("Part 1: {}", solution(d, 0, usize::MAX));
    
    let d = Diagram::parse2(&contents);
    // println!("{}", d);
    println!("Part 2: {}", solution(d, 0, usize::MAX));

    0
}
    