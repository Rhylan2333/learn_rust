#![allow(unused)]
fn main() {
    use std::fmt::Debug;

    fn report<T: Debug>(item: T) {
        println!("{:?}", item);
    }
}

fn clear(text: &mut String) -> () {
    *text = String::from("");
}
