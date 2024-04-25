//还有一些测试用例，用户希望 API 方法基于某些输入而执行失败，并且希望测试框
//架断言此失败。Rust 为此提供了一个名为#[should_panic]的属性。
#[test]
#[should_panic]
fn this_panics() {
    assert_eq!(1, 2);
}