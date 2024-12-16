fn main() {
    let mut a = [0];

    // Rust denies this code at compile time.
    a[3] = 0x7ffff7b36ceb;
    println!("{}", a[3]);
}
