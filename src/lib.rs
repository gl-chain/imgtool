pub fn pow(base: i64, exponent: usize) -> i64 {
    let mut res = 1;
    if exponent == 0 {
        return 1;
    }
    for _ in 0..exponent {
        res *= base as i64;
    }
    res
}

#[cfg(test)]
mod test {

    use super::pow;

    #[test]
    fn it_works() {
        assert_eq!(pow(-2, 3), -8);
    }
}
