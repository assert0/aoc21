use aoc21::days::day5;

#[test]
fn day5_parse_line() {
    let l = day5::VentLine::parse(&String::from("242,601 -> 242,18"));
    assert_eq!(l.start, (242, 601));
    assert_eq!(l.end, (242, 18));
}

