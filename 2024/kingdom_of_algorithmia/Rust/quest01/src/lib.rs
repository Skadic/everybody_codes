pub const INPUT1: &str = include_str!("../input1.txt");
pub const INPUT2: &str = include_str!("../input2.txt");
pub const INPUT3: &str = include_str!("../input3.txt");

pub mod part1;
pub mod part2;
pub mod part3;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) -> Vec<char> {
    input.chars().collect()
}
