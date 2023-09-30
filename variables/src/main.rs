fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;
    let mut y = &x + 1;
    println!("The value of y is: {}", y);
    y = 8;
    println!("The value of y is: {}", y);
}
