use tracing::info;

use crate::{parse_input, INPUT1};

#[tracing::instrument(skip_all, name = "part1", parent=None)]
pub fn process(input: &[char]) -> miette::Result<()> {
    let result = input.iter().fold(0usize, |acc, elem| {
        acc + match elem {
            'B' => 1,
            'C' => 3,
            _ => 0,
        }
    });
    info!(result);
    Ok(())
}

#[cfg(test)]
#[test]
fn part1() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    let input = parse_input(INPUT1);
    process(&input)
}
