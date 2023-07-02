use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn palindrome_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Palindrome");

    let samples = [
        "Madam, I'm Adam!",
        "A man, a plan, a canal: Panama!",
        "Нажал кабан на баклажан.",
    ];

    for sample in samples.iter() {
        group.bench_with_input(
            BenchmarkId::new("is_palindrome_regex", sample),
            sample,
            |b, sample| b.iter(|| palindrome::is_palindrome_regex(sample)),
        );
        group.bench_with_input(
            BenchmarkId::new("is_palindrome_raw", sample),
            sample,
            |b, sample| b.iter(|| palindrome::is_palindrome_raw(sample)),
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    palindrome_benchmark
);

criterion_main!(benches);
