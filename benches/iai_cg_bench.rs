use bitrev::{ReverseBits, ReverseBitsGpt, ReverseBitsLut, ReverseBitsNaive, ReverseBitsSimd};
use iai_callgrind::{black_box, library_benchmark, library_benchmark_group, main};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha12Rng;

// run all the different implementations for a given type of values
macro_rules! have_fun {
    ($t:ty, $val:literal, $std:ident, $base:ident, $simd:ident, $naive:ident, $gpt:ident, $lut:ident) => {
        #[library_benchmark]
        #[bench::short($val)]
        fn $std(value: $t) -> $t {
            black_box(value.reverse_bits())
        }

        #[library_benchmark]
        #[bench::short($val)]
        fn $base(value: $t) -> $t {
            black_box(ReverseBits::reverse_bits_base(value))
        }

        #[library_benchmark]
        #[bench::short($val)]
        fn $simd(value: $t) -> $t {
            black_box(value.reverse_bits_simd())
        }

        #[library_benchmark]
        #[bench::short($val)]
        fn $naive(value: $t) -> $t {
            black_box(value.reverse_bits_naive())
        }

        #[library_benchmark]
        #[bench::short($val)]
        fn $gpt(value: $t) -> $t {
            black_box(value.reverse_bits_gpt())
        }

        #[library_benchmark]
        #[bench::short($val)]
        fn $lut(value: $t) -> $t {
            black_box(value.reverse_bits_lut())
        }
    };
}

have_fun!(u8, 239, std_u8, base_u8, simd_u8, naive_u8, gpt_u8, lut_u8);
have_fun!(u16, 10772, std_u16, base_u16, simd_u16, naive_u16, gpt_u16, lut_u16);
have_fun!(u32, 1991557000, std_u32, base_u32, simd_u32, naive_u32, gpt_u32, lut_u32);
have_fun!(
    u64,
    2455334598189341844,
    std_u64,
    base_u64,
    simd_u64,
    naive_u64,
    gpt_u64,
    lut_u64
);
have_fun!(
    u128,
    294977635522249964106226094345462467956,
    std_u128,
    base_u128,
    simd_u128,
    naive_u128,
    gpt_u128,
    lut_u128
);

library_benchmark_group!(
    name = bench_u8_rev;
    benchmarks = std_u8, base_u8, simd_u8, naive_u8, gpt_u8, lut_u8
);
library_benchmark_group!(
    name = bench_u16_rev;
    benchmarks = std_u16, base_u16, simd_u16, naive_u16, gpt_u16, lut_u16
);
library_benchmark_group!(
    name = bench_u32_rev;
    benchmarks = std_u32, base_u32, simd_u32, naive_u32, gpt_u32, lut_u32
);
library_benchmark_group!(
    name = bench_u64_rev;
    benchmarks = std_u64, base_u64, simd_u64, naive_u64, gpt_u64, lut_u64
);
library_benchmark_group!(
    name = bench_u128_rev;
    benchmarks = std_u128, base_u128, simd_u128, naive_u128, gpt_u128, lut_u128
);

main!(
    library_benchmark_groups = bench_u8_rev,
    bench_u16_rev,
    bench_u32_rev,
    bench_u64_rev,
    bench_u128_rev
);
