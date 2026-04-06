// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num: i32) {
    // type must be known at compile time to use num as range
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
