fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    assert_eq!(ret_unit_type(), ())
}

fn ret_unit_type() {
    let x = 6;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let y = if x % 2 == 1 {
        "odd"
    } else {
        "even"
    };
    println!("x: {}. {}.", x, y);
    // 或者写成一行
    let t = &x +1;
    let z = if t % 2 == 1 { "odd" } else { "even" };
    println!("t: {}. {}.", t, z);
}
