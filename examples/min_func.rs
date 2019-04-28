#[macro_use]
extern crate const_if;

const fn min(x: i32,y: i32) -> i32 {
    const_if!(x < y => x;y)
}


fn main() {
    println!("{}",min(1,2));
}