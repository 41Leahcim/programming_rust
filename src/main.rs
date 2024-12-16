fn set_value(a: &mut [u64], value: u64) {
    // Rust allows this at compile time, but will panic at runtime
    a[3] = value;
    println!("{}", a[3]);
}

fn main() {
    let mut a = [0];

    set_value(&mut a, 0x7ffff7b36ceb);
}
