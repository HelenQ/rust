#[cfg(test)]
mod tests {
    use crate::add_two;

    #[test] // 表示是一个测试方法
    fn add_work() {
        assert_eq!(3, add_two(1, 2))
    }
    #[test]
    #[should_panic(expected = "panic")] // panic 并且包含expected认为通过
    fn panic_work() {
        panic!("panic test");
    }
    // test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
    // 0 measured性能测试
}
