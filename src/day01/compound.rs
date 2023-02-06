pub fn array() {
    println!("===== Array =====");
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {:?}", a); // [42, 42, 42, 42, 42, 0, 42, 42, 42, 42]
}

pub fn tuples() {
    println!("===== Tuples =====");
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0); // 7
    println!("2nd index: {}", t.1); // true
}
