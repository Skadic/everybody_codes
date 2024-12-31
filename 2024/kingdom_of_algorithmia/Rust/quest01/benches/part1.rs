use quest01::{parse_input, INPUT1};

fn main() {
    divan::main();
}

#[divan::bench(sample_size = 100000)]
fn part1(bencher: divan::Bencher) {
    let input = parse_input(INPUT1);
    bencher.bench_local(|| quest01::part1::process(input.as_slice()).unwrap());
}
