//和集成测试的情况类似，在实际运行测试之前，我们可能需要设置一些与安装和拆卸
//有关的代码，通常希望它们由 tests/目录下的所有文件共享。对于共享代码，我们可以将它
//们创建为共享通用代码的文件目录模块，或者使用模块 foo.rs，在我们的集成测试文件中使
//用 mod 关键字来声明和引用它

pub fn setup() {
    println!("Setting up fixtures");
}

pub fn teardown() {
    println!("Tearing down");
}