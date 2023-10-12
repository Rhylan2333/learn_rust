fn main() {
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop(); // 弹出一个字符
    let p2 = string_pop.pop(); // 再弹出一个字符
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
    let mut string_empty = String::from("");
    let p3 = string_empty.pop(); // 无法弹出一个字符
    dbg!(p3);
    dbg!(string_empty);
}
