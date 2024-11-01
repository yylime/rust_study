
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    // should panic 表示允许恐慌
    #[test]
    #[should_panic]
    fn it_works2() {
        // panic!("should panic");
        assert_eq!(2 + 2, 4);
    }
    // 让pannic更加精确
    #[test]
    #[should_panic(expected = "hhahahha")] // except 表示error中必须包含该字符串
    fn it_works3() {
        panic!("assertion failed");
    }
    // 在测试中使用result
    // result 作为返回无需使用should_panic
    #[test]
    fn it_works4() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}