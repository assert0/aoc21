use aoc21::days::day20;

#[test]
fn day20_convert() {
    assert_eq!(34, day20::convert(&String::from("...#...#.").chars().collect()));
}
