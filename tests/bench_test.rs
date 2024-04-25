//rust的基准测试不是稳定版
// #[feature(test)]
// extern crate test;
//
// use test::Bencher;
//
// pub fn do_nothing_slowly() {
//     print!(".");
//     for _ in 1..10_000_000 {};
// }
//
// pub fn do_nothing_fast() {}
//
// // cargo bench运行基准测试代码
// #[bench]
// fn bench_nothing_slowly(b: &mut Bencher) {
//     b.iter(|| do_nothing_slowly());
// }
//
// #[bench]
// fn bench_nothing_fast(b: &mut Bencher) {
//     b.iter(|| do_nothing_fast());
// }