use aho_corasick::AhoCorasick;
use tracing::info;

#[tracing::instrument(skip_all, name = "part1", parent=None)]
pub fn process(input: &[&str], text: &str) -> miette::Result<()> {
    let aho = AhoCorasick::new(input).unwrap();

    let result = aho.find_overlapping_iter(text).count();
    info!(result);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{parse_input, INPUT1};

    use super::process;

    #[test]
    fn part1() -> miette::Result<()> {
        tracing_subscriber::fmt().compact().without_time().init();
        let input = parse_input(INPUT1);
        process(&input.0, input.1)
    }
}
