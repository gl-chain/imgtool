use common::setup;
use common::teardown;

mod common;

//默认情况下，测试工具会在测试函数中隐藏或捕获 println! 语句，以使测试结果更整洁，并仅显示测试工具的输出。
//运行测试时可以添加 --nocapture 参数显示 println! 语句的输出
//cargo test basic_test -- --nocapture
// -- 是 Cargo 自身参数结束的标记，并且之后的任何参数都是传递给 Cargo 调用的二
//进制文件的，该文件由我们的测试工具编译。
#[test]
fn basic_test() {
    setup();
    teardown();
    assert!(true);
}