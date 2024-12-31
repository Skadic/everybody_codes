use quest02::{parse_input, INPUT1};

fn main() {
    divan::main();
}

#[divan::bench(sample_size = 1000)]
fn part1(bencher: divan::Bencher) {
    let input = parse_input(INPUT1);
    bencher.bench_local(|| quest02::part1::process(input.0.as_slice(), input.1).unwrap());
}
