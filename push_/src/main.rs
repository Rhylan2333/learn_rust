fn main() {
    let mut s = String::from("Hello ");
    println!("s = \"Hello \"");

    s.push_str("rust");
    println!("追加字符串（\"rust\"）push_str() -> {}", s);
    s.push('!');
    println!("追加字符（\'!\'）push() -> {}", s);
}
