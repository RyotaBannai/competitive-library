use competitive::{num::*, tools::Xorshift};
use criterion::{BatchSize, Criterion};

pub fn bench_mod_mul(c: &mut Criterion) {
    const A: u32 = 998244353;
    let spec = (..A, ..A);
    let mut group = c.benchmark_group("mod_mul");
    group.bench_function("const_mod_mul", |b| {
        type M = mint_basic::MInt998244353;
        let mut rng = Xorshift::default();
        b.iter_batched(
            || {
                let (a, b) = rng.gen(spec);
                (M::new(a), M::new(b))
            },
            |(a, b)| a * b,
            BatchSize::SmallInput,
        )
    });
    group.bench_function("montgomery_mul", |b| {
        type M = montgomery::MInt998244353;
        let mut rng = Xorshift::default();
        b.iter_batched(
            || {
                let (a, b) = rng.gen(spec);
                (M::new(a), M::new(b))
            },
            |(a, b)| a * b,
            BatchSize::SmallInput,
        )
    });
    group.bench_function("barrett_reduction_rem", |b| {
        let mut rng = Xorshift::default();
        let br = BarrettReduction::<u64>::new(A as _);
        b.iter_batched(
            || {
                let (a, b) = rng.gen(spec);
                (a as u64, b as u64)
            },
            |(a, b)| br.rem(a * b) as u32,
            BatchSize::SmallInput,
        )
    });
    group.finish();
}
