fn main() {
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace); // 我很喜欢`dbg!``
    let a = 1;
    let b = dbg!(a * 2) + 1;
    println!("{}", b);
}
