mod day01; // 查找 day01.rs 或者 day01/mod.rs

pub use crate::day01::compound;
pub use crate::day01::reference;
fn main() {
    println!("===== Day01 =====");
    compound::array();
    compound::tuples();
    reference::references();
}
