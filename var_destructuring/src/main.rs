fn main() {
    let (a, mut b): (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("S1.\na = {:?}, b = {:?}", a, b);
    b = true;
    println!("S2.");
    assert_eq!(a, b);
}
