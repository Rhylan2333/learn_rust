fn main() {
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
    {
        // 字符串类型
        let spaces = "    ";
        println!("spaces_1: {}.", spaces);
        // usize数值类型
        let spaces = spaces.len();
        println!("spaces_2: {}.", spaces);
        // 从此开始报错
        spaces = 2;
        println!("spaces_3: {}.", spaces);
        // 还是报错
        spaces = "  ";
        println!("spaces_4: {}.", spaces);
    }
}
