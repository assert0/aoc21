use std::fs;

pub fn hex_to_bits(hex: char) -> Vec<bool> {
    match hex {
        '0' => vec![false, false, false, false],
        '1' => vec![false, false, false, true],
        '2' => vec![false, false, true, false],
        '3' => vec![false, false, true, true],
        '4' => vec![false, true, false, false],
        '5' => vec![false, true, false, true],
        '6' => vec![false, true, true, false],
        '7' => vec![false, true, true, true],
        '8' => vec![true, false, false, false],
        '9' => vec![true, false, false, true],
        'A' => vec![true, false, true, false],
        'B' => vec![true, false, true, true],
        'C' => vec![true, true, false, false],
        'D' => vec![true, true, false, true],
        'E' => vec![true, true, true, false],
        'F' => vec![true, true, true, true],
        _ => panic!("Unknown char: {}", hex)
    }
}

pub fn bits_to_value(bits: &Vec<bool>) -> usize {
    bits.iter().fold(0, |acc, bit| (acc << 1) | (*bit as usize))
}

pub fn parse_literal(packet: &Vec<bool>) -> (usize, usize) {
    let mut i = 0;
    let mut r: Vec<bool> = vec![];
    loop {
        let more = packet[i];
        r.append(&mut packet[i+1..i+5].to_vec());
        i += 5;
        if !more {
            break;
        }
    }
    (i, bits_to_value(&r))
}

pub fn parse_packet(packet: &Vec<bool>) -> (usize, usize, usize) {
    if packet.len() < 6 {
        return (0, 0, 0);
    }
    let mut i = 0;
    let mut version = bits_to_value(&packet[..3].to_vec());
    i += 3;
    let id = bits_to_value(&packet[i..i+3].to_vec());
    i += 3;
    // println!("ver: {} id: {}", version, id);
    match id {
        4 => {
            let (used, literal) = parse_literal(&packet[i..].to_vec());
            i += used;
            // println!("literal: {}", literal);
            return (i, version, literal);
        },
        _ => {
            // println!("id: {}", id);
            let len_type = packet[i];
            i += 1;
            let len = match len_type {
                false => 15,
                true => 11,
            };
            let value = bits_to_value(&packet[i..i+len].to_vec());
            i += len;
            // println!("len_type: {} value: {}", len_type, value);
            let mut literals = vec![];
            match len_type {
                false => {
                    let mut j = 0;
                    while i + j < i + value {
                        let (k, sub_ver, literal) = parse_packet(&packet[i+j..].to_vec());
                        literals.push(literal);
                        version += sub_ver;
                        j += k;
                    }
                    assert_eq!(j, value);
                    i += value;
                },
                true => {
                    for _ in 0..value {
                        let (j, sub_ver, literal) = parse_packet(&packet[i..].to_vec());
                        literals.push(literal);
                        version += sub_ver;
                        i += j;
                    }
                },
            };
            // println!("literals: {:?}", literals);
            let literal = match id {
                0 => literals.into_iter().sum::<usize>(),
                1 => literals.into_iter().product(),
                2 => literals.into_iter().min().unwrap(),
                3 => literals.into_iter().max().unwrap(),
                5 => if literals[0] > literals[1] { 1 } else { 0 },
                6 => if literals[0] < literals[1] { 1 } else { 0 },
                7 => if literals[0] == literals[1] { 1 } else { 0 },
                _ => 0,
            };
            return (i, version, literal);
        }
    };
}

pub fn day16(args: &[String]) -> i32 {
    println!("Day 16");
    if args.len() != 1 {
        println!("Missing input file");
        return -1;
    }
    let filename = &args[0];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let packet: Vec<_> = contents.chars().map(|c| hex_to_bits(c)).flatten().collect();

    println!("Part 1: {}", parse_packet(&packet).1);
    println!("Part 2: {}", parse_packet(&packet).2);
    0
}
