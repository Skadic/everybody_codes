use std::iter;

use aho_corasick::AhoCorasick;
use fxhash::FxHashSet;
use itertools::Itertools;
use tracing::info;

#[tracing::instrument(skip_all, name = "part3", parent=None)]
pub fn process(input: &[&str], text: &str) -> miette::Result<()> {
    let words = input
        .iter()
        .flat_map(|&s| [s.to_owned(), s.chars().rev().collect::<String>()])
        .collect::<FxHashSet<_>>();
    let original_w = text.lines().next().unwrap().len();
    let original_h = text.lines().count();

    let text = text.lines().flat_map(|line| [line, line, "\n"]).join("");

    let aho = AhoCorasick::new(&words).unwrap();

    let mut positions = FxHashSet::default();

    let w = text.lines().next().unwrap().len() + 1;

    let mut ranges = aho
        .find_overlapping_iter(&text)
        .map(|m| m.range())
        .collect::<Vec<_>>();
    ranges.sort_by_key(|r| r.start);
    for range in ranges {
        let y = range.start / w;
        let x = range.start % w;
        positions.extend(
            (0..range.len())
                .map(|i| (x + i, y))
                .map(|(x, y)| (x % original_w, y % original_h)),
        )
    }

    let text_matrix = text
        .lines()
        .map(str::trim)
        .map(str::chars)
        .map(Itertools::collect_vec)
        .collect_vec();

    let transposed_text = (0..text_matrix[0].len())
        .flat_map(|x| {
            let text_matrix = &text_matrix;
            (0..text_matrix.len())
                .map(move |y| text_matrix[y][x])
                .chain(iter::once('\n'))
        })
        .collect::<String>();

    let w = transposed_text.lines().next().unwrap().len() + 1;

    let mut ranges = aho
        .find_overlapping_iter(&transposed_text)
        .map(|m| m.range())
        .collect::<Vec<_>>();
    ranges.sort_by_key(|r| r.start);
    for range in ranges {
        let y = range.start / w;
        let x = range.start % w;
        if x < original_h && y < original_w {
            positions.extend(
                (0..range.len())
                    .map(|i| (y, x + i))
                    .map(|(x, y)| (x % original_w, y % original_h)),
            )
        }
    }

    info!(result = positions.len());
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{parse_input, INPUT3};

    use super::process;

    #[test]
    fn part3() -> miette::Result<()> {
        tracing_subscriber::fmt().compact().without_time().init();
        let input = parse_input(INPUT3);
        process(&input.0, input.1)
    }
}
