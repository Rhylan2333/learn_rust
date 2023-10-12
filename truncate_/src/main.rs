fn main() {
    let mut string_truncate = String::from("测试truncate");
    // 如果参数所给的位置不是合法的字符边界，则会发生错误。
    string_truncate.truncate(8);
    dbg!(string_truncate);
}
