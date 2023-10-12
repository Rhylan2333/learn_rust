fn main() {
    let mut string_clear = String::from("string clear");
    // 删除字符串中的所有字符，相当于 truncate() 方法参数为 0 的时候。
    string_clear.clear();
    dbg!(string_clear);
}
