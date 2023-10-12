fn main() {
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!"; // `result + "!"` 中的 `result` 是不可变的
    result += "!!!";

    println!("连接字符串 + -> {}", result);

    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    let s3 = s1 + &s2;
    dbg!(assert_eq!(s3, "hello,world!"));
    // 下面的语句如果去掉注释，就会报错
    // println!("{}",s1);
    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");

    // String = String + &str + &str + &str + &str
    let s = s4 + "-" + &s5 + "-" + &s6;
    // 下面的语句如果去掉注释，就会报错
    // dbg!(s4);
    dbg!(s);
}

// fn add(self, s: &str) -> String {}
