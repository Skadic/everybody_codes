use quest01::{parse_input, INPUT2};

fn main() {
    divan::main();
}

#[divan::bench(sample_size = 10000)]
fn part2(bencher: divan::Bencher) {
    let input = parse_input(INPUT2);
    bencher.bench_local(|| quest01::part2::process(input.as_slice()).unwrap());
}