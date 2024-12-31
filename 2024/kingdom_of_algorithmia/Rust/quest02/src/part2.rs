use aho_corasick::AhoCorasick;
use fxhash::FxHashSet;
use tracing::info;

#[tracing::instrument(skip_all, name = "part2", parent=None)]
pub fn process(input: &[&str], text: &str) -> miette::Result<()> {
    let words = input
        .iter()
        .flat_map(|&s| [s.to_owned(), s.chars().rev().collect::<String>()])
        .collect::<FxHashSet<_>>();

    let aho = AhoCorasick::new(&words).unwrap();

    let mut ranges = aho
        .find_overlapping_iter(text)
        .map(|m| m.range())
        .collect::<Vec<_>>();
    ranges.sort_by_key(|r| r.start);
    let mut result = 0;
    let mut last_end = 0;
    for range in ranges {
        result += range.len().min(range.end.saturating_sub(last_end));
        last_end = range.end.max(last_end);
    }
    info!(result);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::process;
    use crate::{parse_input, INPUT2};

    #[test]
    fn part2() -> miette::Result<()> {
        tracing_subscriber::fmt().compact().without_time().init();
        let input = parse_input(INPUT2);
        process(&input.0, input.1)
    }
}
