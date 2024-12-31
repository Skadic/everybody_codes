use tracing::info;

use crate::{parse_input, INPUT3};

#[tracing::instrument(skip_all, name = "part3", parent=None)]
pub fn process(input: &[char]) -> miette::Result<()> {
    let mut num_battle_cnt = [0usize, 0, 0, 0];
    let mut result = input.chunks(3).fold(0usize, |acc, chunk| unsafe {
        let c0 = *chunk.get_unchecked(0);
        let c1 = *chunk.get_unchecked(1);
        let c2 = *chunk.get_unchecked(2);
        let num_battles = (c0 != 'x') as usize + (c1 != 'x') as usize + (c2 != 'x') as usize;
        *num_battle_cnt.get_unchecked_mut(num_battles) += 1;
        acc + map_char(c0) + map_char(c1) + map_char(c2)
    });

    result += num_battle_cnt[2] * 2 + num_battle_cnt[3] * 6;
    info!(result);
    Ok(())
}

fn map_char(c: char) -> usize {
    let mut diff = (c as u8 - b'A') as usize;
    diff >>= diff & 0b1111_1000;
    ((1 << diff) >> 1) | ((diff != 0) as usize)
}

#[cfg(test)]
#[test]
fn part3() -> miette::Result<()> {
    tracing_subscriber::fmt().compact().without_time().init();
    let input = parse_input(INPUT3);
    process(&input)
}
