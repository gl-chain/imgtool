use imgtool::silly_loop;

//编写测试时另一个有用的属性是#[ignore]。如果你的测试代码量非常庞大，那么可以
//使用#[ignore]属性标记告知测试工具在执行 cargo test 命令时忽略此类测试功能。然后你可
//以向测试工具或 cargo test 命令传递--ignored 参数来单独运行这些测试。
#[test]
#[ignore]
fn test_silly_loop() {
    silly_loop();
}