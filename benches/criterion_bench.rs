use bitrev::{ReverseBits, ReverseBitsGpt, ReverseBitsLut, ReverseBitsNaive, ReverseBitsSimd};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;

fn rev_all<T: ReverseBits + Copy>(vals: &[T]) {
    for x in vals {
        black_box(x.reverse_bits_base());
    }
}

fn rev_all_naive<T: ReverseBitsNaive + Copy>(vals: &[T]) {
    for x in vals {
        black_box(x.reverse_bits_naive());
    }
}

fn rev_all_simd<T: ReverseBitsSimd + Copy>(vals: &[T]) {
    for x in vals {
        black_box(x.reverse_bits_simd());
    }
}

fn rev_all_gpt<T: ReverseBitsGpt + Copy>(vals: &[T]) {
    for x in vals {
        black_box(x.reverse_bits_gpt());
    }
}

fn rev_all_lut<T: ReverseBitsLut + Copy>(vals: &[T]) {
    for x in vals {
        black_box(x.reverse_bits_lut());
    }
}

/// Number of integers considered per run
const PER_RUN: usize = 1;

// run all the different implementations for a given type of values
macro_rules! do_a_benchy_bench {
    ($t:ty, $c:expr, $rng:expr) => {{
        let mut group = ($c).benchmark_group(std::stringify!($t));
        let mut vals: [$t; PER_RUN] = [0; PER_RUN];
        ($rng).fill(&mut vals[..]);

        group.bench_function("rev", |b| b.iter(|| rev_all(black_box(&vals))));
        group.bench_function("rev simd", |b| b.iter(|| rev_all_simd(black_box(&vals))));
        group.bench_function("rev gpt", |b| b.iter(|| rev_all_gpt(black_box(&vals))));
        group.bench_function("rev lut", |b| b.iter(|| rev_all_lut(black_box(&vals))));
        group.bench_function("rev naive", |b| b.iter(|| rev_all_naive(black_box(&vals))));

        group.finish();
    }};
}

fn criterion_benchmark(c: &mut Criterion) {
    let seed = 2455334598189341844; // just some random u64

    // We'll generate a bunch of random input data for to test the implementations on
    let mut rng = ChaCha12Rng::seed_from_u64(seed);

    do_a_benchy_bench!(u8, c, rng);
    do_a_benchy_bench!(u16, c, rng);
    do_a_benchy_bench!(u32, c, rng);
    do_a_benchy_bench!(u64, c, rng);
    do_a_benchy_bench!(u128, c, rng);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
