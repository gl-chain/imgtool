#[macro_use]
extern crate criterion;

use criterion::Criterion;

use imgtool::fast_fibonacci;

fn fibonacci_benchmark(c: &mut Criterion) {
    c.bench_function("fib 8", |b| b.iter(|| fast_fibonacci(8)));
}

criterion_group!(fib_bench, fibonacci_benchmark);
criterion_main!(fib_bench);