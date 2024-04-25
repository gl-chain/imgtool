#![doc(html_logo_url = "https://d30y9cdsu7xlg0.cloudfront.net/png/411962-200.png")]

pub mod logic_gates;

pub fn pow(base: i64, exponent: usize) -> i64 {
    let mut res = 1;
    if exponent == 0 {
        return 1;
    }
    for _ in 0..exponent {
        res *= base;
    }
    res
}

pub fn silly_loop() {
    for _ in 1..100_100_100 {}
}

//sum函数是私有的，意味着模块中的单元测试还允许用户测试私有的函数和方法
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub fn slow_fibonacci(nth: usize) -> u64 {
    return if nth <= 1 {
        nth as u64
    } else {
        slow_fibonacci(nth - 1) + slow_fibonacci(nth - 2)
    };
}

pub fn fast_fibonacci(nth: usize) -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _ in 0..nth {
        c = a + b;
        a = b;
        b = c;
    }
    c
}

//#[cfg(...)]属性中的 cfg 通常用于条件编译，但不限于测试代码。
#[cfg(test)]
mod test {
    use super::*;

//将#[cfg(test)] 标记和一个 mod tests 子模块搭配使用，并将所有测试代码及其相关函数封装到此模块中，
    //可以确保代码和生成的二进制文件都是纯粹的测试代码

    #[test]
    fn it_works() {
        assert_eq!(pow(-2, 3), -8);
    }

    fn sum_inputs_outputs() -> Vec<((i32, i32), i32)> {
        vec![((1, 1), 2), ((0, 0), 0), ((-2, 2), 0)]
    }

    #[test]
    fn test_sums() {
        for ((x, y), output) in sum_inputs_outputs() {
            assert_eq!(sum(x, y), output);
        }
    }
}

