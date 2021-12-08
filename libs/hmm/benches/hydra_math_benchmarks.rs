use criterion::{criterion_group, criterion_main};
use criterion::BenchmarkId;
use criterion::Criterion;
use spl_math::precise_number::PreciseNumber;
use uint::construct_uint;

use hydra_math::math::{sqrt, log, checked_pow_fraction};

construct_uint! {
	pub struct U256(4);
}

criterion_group!(benches,
    bench_u128_integer_sqrt,
    bench_u128_natural_log,
    bench_checked_pow_fraction,
);
criterion_main!(benches);

fn bench_u128_integer_sqrt(c: &mut Criterion) {
    let mut group = c.benchmark_group("u128 integer square root");

    for i in [u128::MAX].iter() {
        let i_u128 = i;
        let i_precise = PreciseNumber::new(*i).unwrap();
        let i_u256 = U256::from(*i);
        let parameter = "MAX";

        group.bench_with_input(BenchmarkId::new("u128_Legacy", parameter), &i_u128, |b, &s| {
            b.iter(|| sqrt(*s));
        });

        group.bench_with_input(BenchmarkId::new("u128_PreciseNumber", parameter), &i_precise, |b, s| {
            b.iter(|| s.sqrt().expect("precise_number"));
        });

        group.bench_with_input(BenchmarkId::new("u128_U256", parameter), &i_u256, |b, &s| {
            b.iter(|| s.integer_sqrt().0);
        });
    }
    group.finish();
}

fn bench_u128_natural_log(c: &mut Criterion) {
    let mut group = c.benchmark_group("u128 integer natural log");

    for i in [u64::MAX as u128].iter() {
        let i_u128 = i;
        // TODO: Not implemented
        // let i_precise = PreciseNumber::new(*i).unwrap();
        let parameter = "MAX";

        group.bench_with_input(BenchmarkId::new("u128_Legacy", parameter), &i_u128, |b, &s| {
            b.iter(|| log(*s));
        });
    }
    group.finish();
}

fn bench_checked_pow_fraction(c: &mut Criterion) {
    let mut group = c.benchmark_group("precise number checked power fraction");

    for i in [PreciseNumber::new(u128::MAX).unwrap()].iter() {
        let base = i;
        let parameter = "MAX_BASE";

        let one = PreciseNumber::new(1).unwrap();
        let two = PreciseNumber::new(2).unwrap();
        let three = PreciseNumber::new(3).unwrap();
        let four = PreciseNumber::new(4).unwrap();
        let five = PreciseNumber::new(4).unwrap();

        let base = PreciseNumber::new(42u128).unwrap();
        let exp_0_25 = one.checked_div(&four).unwrap();
        let exp_0_5 = one.checked_div(&two).unwrap();
        let exp_1_25 = five.checked_div(&four).unwrap();
        let exp_1_5 = three.checked_div(&two).unwrap();

        group.bench_with_input(BenchmarkId::new("exp_0_25", parameter), &exp_0_25, |b, s| {
            b.iter(|| checked_pow_fraction(&base, &s),);
        });

        group.bench_with_input(BenchmarkId::new("exp_0_5", parameter), &exp_0_5, |b, s| {
            b.iter(|| checked_pow_fraction(&base, &s),);
        });

        group.bench_with_input(BenchmarkId::new("exp_1_25", parameter), &exp_1_25, |b, s| {
            b.iter(|| checked_pow_fraction(&base, &s),);
        });

        group.bench_with_input(BenchmarkId::new("exp_1_5", parameter), &exp_1_5, |b, s| {
            b.iter(|| checked_pow_fraction(&base, &s),);
        });
    }
    group.finish();
}
