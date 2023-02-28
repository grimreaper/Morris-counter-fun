use ::function_name::named;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

#[named]
pub fn benchmark_nothing(c: &mut Criterion) {
}

criterion_group!(
    benches,
    benchmark_nothing
);
criterion_main!(benches);
